use crate::ast_parser;

pub fn parse(src: String) -> Result<swc_ecma_ast::Module, anyhow::Error> {
    let p = ast_parser::AstParser::new();
    p.parse_module("root.ts", &src, |parse_result| {
        let module = parse_result?;
        Ok(module)
    })
}
