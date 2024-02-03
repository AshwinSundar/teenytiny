mod lex;

fn main() {
    let source: String = String::from("IF+-123 foo*THEN/");
    let mut lexer: lex::Lexer = lex::Lexer::new(source);
    
    let mut t: lex::Token = lexer.get_token();
    println!("{}", t.kind.0);
    while t.kind != lex::TokenKind::EOF {
        t = lexer.get_token();
        println!("{}", t.kind.0);
    }
}

// Next - Part 2