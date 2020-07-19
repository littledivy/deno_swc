use crate::ast_parser::{AstParser, SwcDiagnosticBuffer};
use swc_ecma_ast::{CallExpr, ExportAll, ImportDecl, NamedExport};
use swc_ecma_visit::Node;
use swc_ecma_visit::Visit;

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
import * as spam from "./spam.ts";
import { foo } from "./foo.ts";
export { bar } from "./foo.ts";
export * from "./bar.ts";
"#;

    let dependencies = analyze_dependencies(source, false).expect("Failed to parse");
    assert_eq!(
        dependencies,
        vec![
            "./spam.ts".to_string(),
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
