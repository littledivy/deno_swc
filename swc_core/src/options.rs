use serde::{Deserialize, Serialize};
pub use swc::config::Options;
use swc::ecmascript::ast::Program;

#[derive(Deserialize)]
pub struct ParseArguments {
    pub src: String,
    pub opt: Option<ParseOptions>,
}

#[derive(Deserialize)]
pub struct PrintArguments {
    pub program: Program,
    pub options: Options,
}

#[derive(Deserialize)]
pub struct AnalyzerArguments {
    pub src: String,
    pub dynamic: bool,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ParseOptions {
    #[serde(default)]
    pub comments: bool,
    #[serde(flatten)]
    pub syntax: swc_ecma_parser::Syntax,

    #[serde(default = "default_is_module")]
    pub is_module: bool,

    #[serde(default)]
    pub target: swc_ecma_parser::JscTarget,
}

fn default_is_module() -> bool {
    true
}
