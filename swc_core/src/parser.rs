use swc::common::{errors::Handler, FileName, FilePathMapping, SourceMap};
use swc::config::ParseOptions;
use swc::ecmascript::parser;
use swc::Compiler;
use std::sync::Arc;

fn create_compiler() -> Compiler {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_tty_emitter(
        swc::common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    Compiler::new(cm, handler)
}

pub fn parse_js(src: String) {
    let c = create_compiler();
    let fm = c.cm.new_source_file(FileName::Anon, src);
    let options = ParseOptions {
        comments: true,
        is_module: false,
        syntax: parser::Syntax::default(),
        target: parser::JscTarget::default(),
    };
    let program = c
        .run(|| {
            c.parse_js(
                fm.clone(),
                options.target,
                options.syntax,
                options.is_module,
                options.comments,
            )
        })
        .unwrap();
}

fn parse_ts(src: String) {
    let c = create_compiler();
    let fm =
        c.cm.new_source_file(FileName::Custom("test.ts".into()), src);
    let options = ParseOptions {
        comments: true,
        is_module: true,
        syntax: parser::Syntax::Typescript(std::default::Default::default()),
        target: parser::JscTarget::default(),
    };
    let program = c.run(|| {
        c.parse_js(
            fm.clone(),
            options.target,
            options.syntax,
            options.is_module,
            options.comments,
        )
    });
}
