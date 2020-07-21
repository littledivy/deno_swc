use anyhow::Error;
use std::sync::Arc;
use swc::{
    common::{self, errors::Handler, FilePathMapping, SourceMap, FileName},
    Compiler, TransformOutput,
};

pub fn transform(program_data: String) -> Result<TransformOutput, Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Arc::new(Compiler::new(cm, handler));
    c.run(|| {
        let scf = c.cm.new_source_file(FileName::Anon, program_data);
        c.process_js_file(scf, &Default::default())
    })
}
