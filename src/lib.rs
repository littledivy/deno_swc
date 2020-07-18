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
    interface.register_op("parse", op_parse);
}

struct ParseTask {
    c: Arc<Compiler>,
    fm: Arc<SourceFile>,
    options: ParseOptions,
}

#[derive(Deserialize)]
struct ParseArguments {
    src: String,
}

fn op_parse(_interface: &mut dyn Interface,  zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let cm = Arc::new(SourceMap::new(FilePathMapping::empty()));
    let params: ParseArguments = serde_json::from_slice(&data).unwrap();
    let handler = Arc::new(Handler::with_tty_emitter(
        common::errors::ColorConfig::Always,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Compiler::new(cm.clone(), handler);
    let fm = c.cm.new_source_file(FileName::Anon, params.src.to_string());
    let options = ParseOptions {
        comments: true,
        is_module: false,
        syntax: swc::ecmascript::parser::Syntax::default(),
        target: swc::ecmascript::parser::JscTarget::default()
    };
    let program = c.run(|| {
        c.parse_js(
            fm.clone(),
            options.target,
            options.syntax,
            options.is_module,
            options.comments,
        )
    }).unwrap();
    let result = serde_json::to_string(&program).expect("failed to serialize Program");
    let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}
