use std::sync::Arc;
use swc::common::{errors::Handler, FileName, FilePathMapping, SourceMap};
use swc::config::ParseOptions;
use swc::ecmascript::parser;
use swc::Compiler;

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

pub fn parse_ts(src: String) -> Result<swc::ecmascript::ast::Program, anyhow::Error> {
    let p = ast_parser::AstParser::new();
    p.parse_module("root.ts", src, |parse_result| {
        let module = parse_result?;
        Ok(module)
    })
}
