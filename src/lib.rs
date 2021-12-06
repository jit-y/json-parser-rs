mod lexer;
mod parser;

pub fn print_json_ast(input: &str) -> parser::Result<()> {
    let l = lexer::Lexer::new(input);
    let mut p = parser::Parser::new(l);

    println!("{:?}", p.parse()?);

    Ok(())
}
