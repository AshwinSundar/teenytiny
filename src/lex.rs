use std::process::exit;

pub struct Lexer {
  pub source: Vec<char>, // Source code to lex as a vector of characters
  pub cur_char: char, // Current character in the vector
  pub cur_pos: i32, // Current position in the string
}

impl Lexer { 
  pub fn new(src: String) -> Lexer {
    let mut src_vec: Vec<char> = src.chars().collect();
    src_vec.push('\n');
    let cur_char: char = src_vec.get(0).cloned().unwrap_or('\0');
    Lexer {
      source: src_vec, // accept string as arg to the construct, and collect into vector of chars here
      cur_char: cur_char,
      cur_pos: 0,
    }
  }

  pub fn next_char(&mut self) {
    self.cur_pos += 1;
    if self.cur_pos >= self.source.len() as i32 { 
      self.cur_char = '\0'; 
    }
    else {
      self.cur_char = self.source[self.cur_pos as usize];
    }
  }

  pub fn peek(&self) -> char {
    if self.cur_pos + 1 >= self.source.len() as i32 {
      '\0'
    }
    else {
      match self.source.get((self.cur_pos + 1) as usize) {
        Some(&c) => c,
        None => '\0', 
      }
    }
  }

  pub fn abort(&self, msg: String) {
    print!("{}", msg);
    exit(1)
  }
  
  pub fn skip_whitespace(&mut self) {
    while self.cur_char == ' ' || self.cur_char == '\t' || self.cur_char == '\r' {
      self.next_char()
    }
  }
  
  pub fn skip_comment(&mut self) {
    if self.cur_char == '#' {
      while self.cur_char != '\n' {
        self.next_char();
      }
    }
    
  }
  
  pub fn get_token(&mut self) -> Token {
    self.skip_whitespace();
    self.skip_comment();
    let t: Token;

    match (self.cur_char, self.peek()) {
      ('+', _) => { t = Token{text: self.cur_char.to_string(), token_type: TokenKind::PLUS}; }, // Plus token
      ('-', _) => { t = Token{text: self.cur_char.to_string(), token_type: TokenKind::MINUS}; }, // Minus token
      ('*', _) => { t = Token{text: self.cur_char.to_string(), token_type: TokenKind::ASTERISK}; }, // Asterisk token
      ('/', _) => { t = Token{text: self.cur_char.to_string(), token_type: TokenKind::SLASH}; }, // Slash token
      ('\n', _) => { t = Token{text: self.cur_char.to_string(), token_type: TokenKind::NEWLINE}; }, // Newline token
      ('\0', _) => { t = Token{text: self.cur_char.to_string(), token_type: TokenKind::EOF}; }, // EOF token
      ('=', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, token_type: TokenKind::EQEQ};
        }
        else {
          t = Token{text: self.cur_char.to_string(), token_type: TokenKind::EQ};
        }
      },
      ('>', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, token_type: TokenKind::GTEQ};
        }
        else {
          t = Token{text: self.cur_char.to_string(), token_type: TokenKind::GT};
        }
      },
      ('<', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, token_type: TokenKind::LTEQ};
        }
        else {
          t = Token{text: self.cur_char.to_string(), token_type: TokenKind::LT};
        }
      },
      ('!', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, token_type: TokenKind::NOTEQ};
        }
        else {
          t = Token{ text: self.cur_char.to_string(), token_type: TokenKind::LEX_ERR}; // Unknown token
          let mut msg = String::from("Lexing error. Expected !=, got !");
          msg.push_str(&n.to_string());
          self.abort(msg);
        }
      },
      ('\"', _) => {
        self.next_char();
        let mut substring = self.cur_char.to_string();

        while self.cur_char != '\"' {
          if self.cur_char == '\r' || self.cur_char == '\n' || self.cur_char == '\t' || self.cur_char == '\\' || self.cur_char == '%' {
              self.abort(String::from("Illegal character in string."))
          }

          self.next_char();
          substring.push(self.cur_char);
        }

        t = Token{text: substring, token_type: TokenKind::STRING};
      },
      (c, _) if c.is_numeric() => { // This case consists of some ugly code...feels way too verbose and nested
        let mut substring = self.cur_char.to_string();

        while self.peek().is_numeric() {
          substring.push(self.peek());
          self.next_char();
        }

        if self.peek() == '.' {
          substring.push(self.peek());
          self.next_char();

          if !self.peek().is_numeric() {
            self.abort(String::from("Illegal character in number."))
          }

          while self.peek().is_numeric() {
            substring.push(self.peek());
            self.next_char();
          }
        }

        t = Token{text: substring, token_type: TokenKind::NUMBER};
      },
      (c, _) if c.is_alphabetic() => {
        let mut substring = self.cur_char.to_string();

        while self.peek().is_alphabetic() {
          substring.push(self.peek());
          self.next_char();
        }
        
        let keyword = Token::check_if_keyword(&substring);
        match keyword {
          Some(k) => t = Token{text: substring, token_type: k},
          None => t = Token{text: substring, token_type: TokenKind::IDENT}
        }
      }
      _ => { 
        t = Token{text: self.cur_char.to_string(), token_type: TokenKind::LEX_ERR}; // Unknown token
        let mut msg = String::from("Lexing error. ");
        msg.push_str(&self.cur_char.to_string());
        self.abort(msg);
      }
    }

    self.next_char(); // should refactor this side effect out nicely...
    t
  }
}


pub type TokenType = (&'static str, i16);

// Token is an enum of type TokenType = (&'static str, i16)
// The actual enum values are defined in TokenKind
#[derive(Clone)]
pub struct Token {
  pub text: String,
  pub token_type: TokenType,
}

impl Token {
  pub fn check_if_keyword(token_text: &String) -> Option<(&'static str, i16)> {
    for k in TokenKind::ALL {
      if k.0 == token_text && k.1 >= 100 && k.1 < 200 {
        return Some(k)
      }
    }
    return None
  }
}

pub struct TokenKind {
  // pub text: String,
  pub name: &'static str,
  pub val: i16,
}

impl TokenKind {
  pub const NONE: TokenType = ("NONE", -3);
  pub const LEX_ERR: TokenType = ("LEX_ERR", -2);
  pub const EOF: TokenType = ("EOF", -1);
	pub const NEWLINE: TokenType = ("NEWLINE", 0);
	pub const NUMBER: TokenType = ("NUMBER", 1);
	pub const IDENT: TokenType = ("IDENT", 2);
	pub const STRING: TokenType = ("STRING", 3);
	// Keywords.
	pub const LABEL: TokenType = ("LABEL", 101);
	pub const GOTO: TokenType = ("GOTO", 102);
	pub const PRINT: TokenType = ("PRINT", 103);
	pub const INPUT: TokenType = ("INPUT", 104);
	pub const LET: TokenType = ("LET", 105);
	pub const IF: TokenType = ("IF", 106);
	pub const THEN: TokenType = ("THEN", 107);
	pub const ENDIF: TokenType = ("ENDIF", 108);
	pub const WHILE: TokenType = ("WHILE", 109);
	pub const REPEAT: TokenType = ("REPEAT", 110);
	pub const ENDWHILE: TokenType = ("ENDWHILE", 111);
	// Operators.
	pub const EQ: TokenType = ("EQ", 201);
	pub const PLUS: TokenType = ("PLUS", 202);
	pub const MINUS: TokenType = ("MINUS", 203);
	pub const ASTERISK: TokenType = ("ASTERISK", 204);
	pub const SLASH: TokenType = ("SLASH", 205);
	pub const EQEQ: TokenType = ("EQEQ", 206);
	pub const NOTEQ: TokenType = ("NOTEQ", 207);
	pub const LT: TokenType = ("LT", 208);
	pub const LTEQ: TokenType = ("LTEQ", 209);
	pub const GT: TokenType = ("GT", 210);
	pub const GTEQ: TokenType = ("GTEQ", 211);
  

  pub const ALL: [TokenType; 29] = [
    Self::NONE,
    Self::LEX_ERR,
    Self::EOF,
    Self::NEWLINE,
    Self::NUMBER,
    Self::IDENT,
    Self::STRING,
    Self::LABEL,
    Self::GOTO,
    Self::PRINT,
    Self::INPUT,
    Self::LET,
    Self::IF,
    Self::THEN,
    Self::ENDIF,
    Self::WHILE,
    Self::REPEAT,
    Self::ENDWHILE,
    Self::EQ,
    Self::PLUS,
    Self::MINUS,
    Self::ASTERISK,
    Self::SLASH,
    Self::EQEQ,
    Self::NOTEQ,
    Self::LT,
    Self::LTEQ,
    Self::GT,
    Self::GTEQ,
  ];
}
