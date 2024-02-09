use std::fs;

mod lex;
mod parse;

fn main() {
    println!("Teeny Tiny Compiler 0.1");
    let source = fs::read_to_string("src/hello.teeny").expect("File not found");
    let lexer: lex::Lexer = lex::Lexer::new(source);
    let mut parser: parse::Parser = parse::Parser::new(lexer);

    parser.program();
    println!("Parsing completed.");
    
}
// next - part 3