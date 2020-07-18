use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use serde::Deserialize;
use serde::Serialize;
use std::{
    env,
    panic::set_hook,
    path::{Path, PathBuf},
    sync::Arc,
};
use swc::{
    common::{self, errors::Handler, FileName, FilePathMapping, SourceFile, SourceMap},
    config::{Options, ParseOptions, SourceMapsConfig},
    ecmascript::ast::Program,
    Compiler, TransformOutput,
};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("parse", op_compile);
}

struct ParseTask {
    c: Arc<Compiler>,
    fm: Arc<SourceFile>,
    options: ParseOptions,
}

fn op_parse(_interface: &mut dyn Interface, data: &[u8], _zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));

    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Compiler::new(cm.clone(), handler);
    let fm = c.cm.new_source_file(FileName::Anon, src.value());
    c.run(|| {
        c.parse_js(
            fm.clone(),
            self.options.target,
            self.options.syntax,
            self.options.is_module,
            self.options.comments,
        )
    });
    let result_box: Buf = serde_json::to_vec(&response).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}