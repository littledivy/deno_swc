use anyhow::{bail, Error};
use spack::{
    loaders,
    resolve::NodeResolver,
    BundleKind,
};
use std::{
    panic::{catch_unwind, AssertUnwindSafe},
    sync::Arc,
};
use swc::{
    common::{self, errors::Handler, FilePathMapping, SourceMap},
    Compiler,
};
pub fn bundle(data: &[u8]) -> Result<Vec<spack::Bundle>, Error> {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let _params: serde_json::Value = serde_json::from_slice(&data).unwrap();
    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let r = box NodeResolver as Box<_>;
    let c = Arc::new(Compiler::new(cm.clone(), handler));
    let loader = loaders::swc::SwcLoader::new(c.clone(), Default::default());
    let res = catch_unwind(AssertUnwindSafe(|| {
            let bundler = spack::Bundler::new(
                c.clone(),
                serde_json::from_value(serde_json::Value::Object(Default::default()))
                    .unwrap(),
                &r,
                &loader,
            );

            let result = bundler.bundle(&spack::config::Config {
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
    }));

    let err = match res {
        Ok(v) => return v,
        Err(err) => err,
    };

    if let Some(s) = err.downcast_ref::<String>() {
        bail!("panic detected: {}", s);
    }

    bail!("panic detected")
}
