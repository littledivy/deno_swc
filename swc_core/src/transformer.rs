use anyhow::Error;
use std::sync::Arc;
use std::path::PathBuf;
use swc::{
    common::{self, errors::Handler, FilePathMapping, SourceMap},
    config::SourceMapsConfig,
    ecmascript::ast::Program,
    Compiler, TransformOutput,
};

pub fn transform(program_data: String) -> Result<TransformOutput, Error> {
    let program: Program =
        serde_json::from_str(&program_data).expect("failed to deserialize Program");
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Arc::new(Compiler::new(cm, handler));
    c.run(|| {
        c.process_js(program, &Default::default())
    })
}
