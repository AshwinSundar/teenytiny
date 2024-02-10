use std::fs;

mod lex;
mod parse;
mod emit;

fn main() {
    println!("Teeny Tiny Compiler 0.1");
    // let source = fs::read_to_string("src/hello.teeny").expect("File not found");
    let source = fs::read_to_string("src/average.teeny").expect("File not found");
    let lexer: lex::Lexer = lex::Lexer::new(source);
    let mut emitter: emit::Emitter = emit::Emitter::new(String::from("out.c"));
    let mut parser: parse::Parser = parse::Parser::new(lexer, &mut emitter);

    parser.program(); // Run the parser
    emitter.write_file();
    println!("Compiling completed.");
}
