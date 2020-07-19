use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use serde::Deserialize;

use std::sync::Arc;

use core::{analyzer, parser};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("parse", op_parse);
    interface.register_op("parse_ts", op_parse_ts);
    interface.register_op("analyze", op_deps_analyzer);
}

#[derive(Deserialize)]
struct ParseArguments {
    src: String,
}

#[derive(Deserialize)]
struct AnalyzerArguments {
    src: String,
    dynamic: bool,
}

fn op_deps_analyzer(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: AnalyzerArguments = serde_json::from_slice(&data).unwrap();
    let deps =
        analyzer::analyze_dependencies(&params.src, params.dynamic).expect("Failed to parse");
    let result = serde_json::to_string(&deps).expect("failed to serialize Deps");
    let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

fn op_parse(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: ParseArguments = serde_json::from_slice(&data).unwrap();
    let program = parser::parse_js(params.src);
    let result = serde_json::to_string(&program).expect("failed to serialize Program");
    let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}

fn op_parse_ts(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: ParseArguments = serde_json::from_slice(&data).unwrap();
    let program = parser::parse_ts(params.src);
    let result = match program {
        Ok(ast) => serde_json::to_string(&ast).expect("failed to serialize Program"),
        Err(message) => {
            serde_json::to_string(&message.to_string()).expect("failed to serialize Program")
        }
    };
    let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
    Op::Sync(result_box)
}
