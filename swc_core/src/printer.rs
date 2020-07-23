use anyhow::Error;
use std::sync::Arc;
use crate::options::PrintArguments;
use swc::{
    common::{self, errors::Handler, FilePathMapping, SourceMap},
    config::SourceMapsConfig,
    Compiler, TransformOutput,
};

pub fn print(program_data: PrintArguments) -> Result<TransformOutput, Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Arc::new(Compiler::new(cm, handler));
    c.run(|| {
        c.print(
            &program_data.program,
            program_data
                .options
                .source_maps
                .clone()
                .unwrap_or(SourceMapsConfig::Bool(false)),
            None,
            program_data
                    .options
                    .config
                    .clone()
                    .unwrap_or_default()
                    .minify
                    .unwrap_or(false)
        )
    })
}
