use std::error::Error;
use std::fmt;
use std::sync::Arc;
use std::sync::RwLock;
use swc_common::errors::Emitter;
use swc_common::{
    self,
    comments::Comments,
    errors::{Diagnostic, DiagnosticBuilder, Handler, HandlerFlags},
    FileName, Globals, SourceMap, Span,
};
use swc_ecma_ast::{CallExpr, ExportAll, ImportDecl, NamedExport};
use swc_ecma_parser::{
    lexer::Lexer, JscTarget, Parser, Session, SourceFileInput, Syntax, TsConfig,
};
use swc_ecma_visit::Node;
use swc_ecma_visit::Visit;

#[derive(Clone, Debug)]
pub struct SwcDiagnosticBuffer {
    pub diagnostics: Vec<Diagnostic>,
}

impl Error for SwcDiagnosticBuffer {}

impl fmt::Display for SwcDiagnosticBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = self
            .diagnostics
            .iter()
            .map(|d| d.message())
            .collect::<Vec<String>>()
            .join(",");

        f.pad(&msg)
    }
}

#[derive(Clone)]
pub struct SwcErrorBuffer(Arc<RwLock<SwcDiagnosticBuffer>>);

impl SwcErrorBuffer {
    pub fn default() -> Self {
        Self(Arc::new(RwLock::new(SwcDiagnosticBuffer {
            diagnostics: vec![],
        })))
    }
}

impl Emitter for SwcErrorBuffer {
    fn emit(&mut self, db: &DiagnosticBuilder) {
        self.0.write().unwrap().diagnostics.push((**db).clone());
    }
}

impl From<SwcErrorBuffer> for SwcDiagnosticBuffer {
    fn from(buf: SwcErrorBuffer) -> Self {
        let s = buf.0.read().unwrap();
        s.clone()
    }
}

/// Low-level utility structure with common AST parsing functions.
///
/// Allows to build more complicated parser by providing a callback
/// to `parse_module`.
pub struct AstParser {
    pub buffered_error: SwcErrorBuffer,
    pub source_map: Arc<SourceMap>,
    pub handler: Handler,
    pub comments: Comments,
    pub globals: Globals,
}

impl AstParser {
    pub fn new() -> Self {
        let buffered_error = SwcErrorBuffer::default();

        let handler = Handler::with_emitter_and_flags(
            Box::new(buffered_error.clone()),
            HandlerFlags {
                dont_buffer_diagnostics: true,
                can_emit_warnings: true,
                ..Default::default()
            },
        );

        AstParser {
            buffered_error,
            source_map: Arc::new(SourceMap::default()),
            handler,
            comments: Comments::default(),
            globals: Globals::new(),
        }
    }

    pub fn parse_module<F, R>(&self, file_name: &str, source_code: &str, callback: F) -> R
    where
        F: FnOnce(Result<swc_ecma_ast::Module, SwcDiagnosticBuffer>) -> R,
    {
        swc_common::GLOBALS.set(&self.globals, || {
            let swc_source_file = self.source_map.new_source_file(
                FileName::Custom(file_name.to_string()),
                source_code.to_string(),
            );

            let buffered_err = self.buffered_error.clone();
            let session = Session {
                handler: &self.handler,
            };

            let mut ts_config = TsConfig::default();
            ts_config.dynamic_import = true;
            let syntax = Syntax::Typescript(ts_config);

            let lexer = Lexer::new(
                session,
                syntax,
                JscTarget::Es2019,
                SourceFileInput::from(&*swc_source_file),
                Some(&self.comments),
            );

            let mut parser = Parser::new_from(session, lexer);

            let parse_result = parser
                .parse_module()
                .map_err(move |mut err: DiagnosticBuilder| {
                    err.cancel();
                    SwcDiagnosticBuffer::from(buffered_err)
                });

            callback(parse_result)
        })
    }

    pub fn get_span_location(&self, span: Span) -> swc_common::Loc {
        self.source_map.lookup_char_pos(span.lo())
    }

    pub fn get_span_comments(&self, span: Span) -> Vec<swc_common::comments::Comment> {
        self.comments
            .take_leading_comments(span.lo())
            .unwrap_or_else(|| vec![])
    }
}

struct DependencyVisitor {
    dependencies: Vec<String>,
    analyze_dynamic_imports: bool,
}

impl Visit for DependencyVisitor {
    fn visit_import_decl(&mut self, import_decl: &ImportDecl, _parent: &dyn Node) {
        let src_str = import_decl.src.value.to_string();
        self.dependencies.push(src_str);
    }

    fn visit_named_export(&mut self, named_export: &NamedExport, _parent: &dyn Node) {
        if let Some(src) = &named_export.src {
            let src_str = src.value.to_string();
            self.dependencies.push(src_str);
        }
    }

    fn visit_export_all(&mut self, export_all: &ExportAll, _parent: &dyn Node) {
        let src_str = export_all.src.value.to_string();
        self.dependencies.push(src_str);
    }

    fn visit_call_expr(&mut self, call_expr: &CallExpr, _parent: &dyn Node) {
        use swc_ecma_ast::{Expr::*, ExprOrSuper::*, Lit};
        if !self.analyze_dynamic_imports {
            return;
        }

        let boxed_expr = match call_expr.callee.clone() {
            Super(_) => return,
            Expr(boxed) => boxed,
        };

        match &*boxed_expr {
            Ident(ident) => {
                if &ident.sym.to_string() != "import" {
                    return;
                }
            }
            _ => return,
        };

        if let Some(arg) = call_expr.args.get(0) {
            match &*arg.expr {
                Lit(lit) => {
                    if let Lit::Str(str_) = lit {
                        let src_str = str_.value.to_string();
                        self.dependencies.push(src_str);
                    }
                }
                _ => return,
            }
        }
    }
}

#[allow(unused)]
pub fn analyze_dependencies(
    source_code: &str,
    analyze_dynamic_imports: bool,
) -> Result<Vec<String>, SwcDiagnosticBuffer> {
    let parser = AstParser::new();
    parser.parse_module("root.ts", source_code, |parse_result| {
        let module = parse_result?;
        let mut collector = DependencyVisitor {
            dependencies: vec![],
            analyze_dynamic_imports,
        };
        collector.visit_module(&module, &module);
        Ok(collector.dependencies)
    })
}

#[test]
fn test_analyze_dependencies() {
    let source = r#"
import { foo } from "./foo.ts";
export { bar } from "./foo.ts";
export * from "./bar.ts";
"#;

    let dependencies = analyze_dependencies(source, false).expect("Failed to parse");
    assert_eq!(
        dependencies,
        vec![
            "./foo.ts".to_string(),
            "./foo.ts".to_string(),
            "./bar.ts".to_string(),
        ]
    );
}

#[test]
fn test_analyze_dependencies_dyn_imports() {
    let source = r#"
import { foo } from "./foo.ts";
export { bar } from "./foo.ts";
export * from "./bar.ts";
const a = await import("./fizz.ts");
const a = await import("./" + "buzz.ts");
"#;

    let dependencies = analyze_dependencies(source, true).expect("Failed to parse");
    assert_eq!(
        dependencies,
        vec![
            "./foo.ts".to_string(),
            "./foo.ts".to_string(),
            "./bar.ts".to_string(),
            "./fizz.ts".to_string(),
        ]
    );
}
