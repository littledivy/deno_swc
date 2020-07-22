use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct ParseArguments {
    pub src: String,
    pub opt: Option<ParseOptions>,
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
