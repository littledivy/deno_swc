use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use serde::Deserialize;

use std::sync::Arc;

use swc::common::{self, errors::Handler, FileName, FilePathMapping, SourceMap};
use swc::config::ParseOptions;
use swc::ecmascript::parser;
use swc::Compiler;
mod analyzer;
mod ast_parser;

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("parse", op_parse);
    interface.register_op("parse_ts", op_parse_ts);
}

#[derive(Deserialize)]
struct ParseArguments {
    src: String,
}

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

fn op_parse(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: ParseArguments = serde_json::from_slice(&data).unwrap();
    let c = create_compiler();
    let fm = c.cm.new_source_file(FileName::Anon, params.src);
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
    let result = serde_json::to_string(&program).expect("failed to serialize Program");
    let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

fn op_parse_ts(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: ParseArguments = serde_json::from_slice(&data).unwrap();
    let c = create_compiler();
    let fm =
        c.cm.new_source_file(FileName::Custom("test.ts".into()), params.src);
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
    let result = match program {
        Ok(ast) => serde_json::to_string(&ast).expect("failed to serialize Program"),
        Err(message) => {
            serde_json::to_string(&message.to_string()).expect("failed to serialize Program")
        }
    };
    let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}
