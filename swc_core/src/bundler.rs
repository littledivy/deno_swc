use anyhow::{bail, Error};
use spack::{
    load::Load,
    loaders,
    resolve::{NodeResolver, Resolve},
    BundleKind,
};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};
use swc::{
    common::{self, errors::Handler, FileName, FilePathMapping, SourceFile, SourceMap},
    config::{Options, ParseOptions, SourceMapsConfig},
    ecmascript::ast::Program,
    Compiler, TransformOutput,
};
pub fn bundle(data: &[u8]) -> Result<Vec<spack::Bundle>, Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let params: serde_json::Value = serde_json::from_slice(&data).unwrap();
    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Arc::new(Compiler::new(cm.clone(), handler));

    let bundler = spack::Bundler::new(
                c.clone(),
                serde_json::from_value(serde_json::Value::Object(Default::default()))
                    .unwrap(),
                &(box NodeResolver as Box<_>),
                &loaders::swc::SwcLoader::new(c.clone(), Default::default()),
            );

            let result = &bundler.bundle(&spack::config::Config {
                working_dir: std::path::PathBuf::from("./"),
                mode: spack::config::Mode::Production,
                module: spack::config::ModuleConfig { },
                optimization: None,
                resolve: None,
                options: None,
                output: None,
                entry: spack::config::EntryConfig::File("asd".to_string()),
            })?;
            Ok(result)
}
