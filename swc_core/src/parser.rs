use crate::ast_parser;
use crate::options::ParseOptions;

pub fn parse(
    src: String,
    parse_options: ParseOptions,
) -> Result<swc_ecma_ast::Module, anyhow::Error> {
    let p = ast_parser::AstParser::new();
    p.parse_module("root.ts", &src, parse_options, |parse_result| {
        let module = parse_result?;
        Ok(module)
    })
}
