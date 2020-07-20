use deno_core::plugin_api::Buf;
use deno_core::plugin_api::Interface;
use deno_core::plugin_api::Op;
use deno_core::plugin_api::ZeroCopyBuf;

use serde::Deserialize;

use core::{analyzer, parser, printer};

#[no_mangle]
pub fn deno_plugin_init(interface: &mut dyn Interface) {
    interface.register_op("parse", op_parse);
    interface.register_op("print", op_print);
    interface.register_op("extract_dependencies", ops_extract_dependencies);
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

#[allow(clippy::needless_return)]
fn ops_extract_dependencies(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: AnalyzerArguments = serde_json::from_slice(&data).unwrap();
    match analyzer::analyze_dependencies(&params.src, params.dynamic) {
        Ok(deps) => {
            let result = serde_json::to_string(&deps).expect("failed to serialize Deps");
            let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
            Op::Sync(result_box)
        }
        Err(_) => {
            //TODO: return actual error message instead of "parse_error"
            let result = serde_json::to_string("parse_error").expect("failed to serialize Deps");
            let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
            Op::Sync(result_box)
        }
    }
}

fn op_print(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let prg: serde_json::Value = serde_json::from_slice(&data).unwrap();
    match printer::print(prg.to_string()) {
        Ok(program) => {
            let result = serde_json::to_string(&program).expect("failed to serialize Program");
            let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
            Op::Sync(result_box)
        }
        Err(e) => {
            let result =
                serde_json::to_string(&e.to_string()).expect("failed to serialize Program");
            let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
            Op::Sync(result_box)
        }
    }
}

fn op_parse(_interface: &mut dyn Interface, zero_copy: &mut [ZeroCopyBuf]) -> Op {
    let data = &zero_copy[0][..];
    let params: ParseArguments = serde_json::from_slice(&data).unwrap();
    match parser::parse(params.src) {
        Ok(program) => {
            let result = serde_json::to_string(&program).expect("failed to serialize Program");
            let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
            Op::Sync(result_box)
        }
        Err(e) => {
            let result =
                serde_json::to_string(&e.to_string()).expect("failed to serialize Program");
            let result_box: Buf = serde_json::to_vec(&result).unwrap().into_boxed_slice();
            Op::Sync(result_box)
        }
    }
}
