#![feature(box_syntax)]
#![feature(box_patterns)]
mod analyzer;
mod ast_parser;
mod options;

use console_error_panic_hook::hook;
use js_sys::Array;
use std::panic;
use wasm_bindgen::prelude::{wasm_bindgen, JsValue};
use swc_common::{
    errors::{DiagnosticBuilder, Emitter, Handler, SourceMapperDyn},
    FileName, FilePathMapping, SourceMap,
};
use swc_ecmascript::ast::Program;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn parse_sync(s: &str, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let opts: ParseOptions = opts
        .into_serde()
        .map_err(|err| format!("failed to parse options: {}", err))?;

    let (c, errors) = compiler();

    let fm = c.cm.new_source_file(FileName::Anon, s.into());
    let program = c
        .parse_js(fm, opts.target, opts.syntax, opts.is_module, opts.comments)
        .map_err(|err| format!("failed to parse: {}\n{}", err, errors))?;

    Ok(JsValue::from_serde(&program).map_err(|err| format!("failed to return value: {}", err))?)
}

fn compiler() -> (Compiler, BufferedError) {
    let cm = codemap();

    let (handler, errors) = new_handler(cm.clone());

    let c = Compiler::new(cm.clone(), handler);

    (c, errors)
}

/// Get global sourcemap
fn codemap() -> Arc<SourceMap> {
    static CM: Lazy<Arc<SourceMap>> =
        Lazy::new(|| Arc::new(SourceMap::new(FilePathMapping::empty())));

    CM.clone()
}

/// Creates a new handler which emits to returned buffer.
fn new_handler(_cm: Arc<SourceMapperDyn>) -> (Arc<Handler>, BufferedError) {
    let e = BufferedError::default();

    let handler = Handler::with_emitter(true, false, Box::new(MyEmiter::default()));

    (Arc::new(handler), e)
}


#[wasm_bindgen(js_name = "printSync")]
pub fn print_sync(s: JsValue, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let program: Program = s
        .into_serde()
        .map_err(|err| format!("not a program: {}", err))?;

    let opts: Options = opts
        .into_serde()
        .map_err(|err| format!("failed to parse options: {}", err))?;

    let (c, errors) = compiler();

    let s = c
        .print(
            &program,
            opts.source_maps
                .clone()
                .unwrap_or(SourceMapsConfig::Bool(false)),
            None,
            opts.config.unwrap_or_default().minify.unwrap_or_default(),
        )
        .map_err(|err| format!("failed to print: {}\n{}", err, errors))?;

    Ok(JsValue::from_serde(&s).map_err(|err| format!("failed to print: {}\n{}", err, errors))?)
}

#[wasm_bindgen(js_name = "transformSync")]
pub fn transform_sync(s: &str, opts: JsValue) -> Result<JsValue, JsValue> {
    console_error_panic_hook::set_once();

    let opts: Options = opts
        .into_serde()
        .map_err(|err| format!("failed to parse options: {}", err))?;

    let (c, errors) = compiler();

    let fm = c.cm.new_source_file(FileName::Anon, s.into());
    let out = c
        .process_js_file(fm, &opts)
        .map_err(|err| format!("failed to process code: {}\n{}", err, errors))?;

    Ok(JsValue::from_serde(&out).unwrap())
}
