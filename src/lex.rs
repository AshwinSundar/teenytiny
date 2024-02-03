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
      ('+', _) => { t = Token{text: self.cur_char.to_string(), kind: TokenKind::PLUS}; }, // Plus token
      ('-', _) => { t = Token{text: self.cur_char.to_string(), kind: TokenKind::MINUS}; }, // Minus token
      ('*', _) => { t = Token{text: self.cur_char.to_string(), kind: TokenKind::ASTERISK}; }, // Asterisk token
      ('/', _) => { t = Token{text: self.cur_char.to_string(), kind: TokenKind::SLASH}; }, // Slash token
      ('\n', _) => { t = Token{text: self.cur_char.to_string(), kind: TokenKind::NEWLINE}; }, // Newline token
      ('\0', _) => { t = Token{text: self.cur_char.to_string(), kind: TokenKind::EOF}; }, // EOF token
      ('=', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, kind: TokenKind::EQEQ};
        }
        else {
          t = Token{text: self.cur_char.to_string(), kind: TokenKind::EQ};
        }
      },
      ('>', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, kind: TokenKind::GTEQ};
        }
        else {
          t = Token{text: self.cur_char.to_string(), kind: TokenKind::GT};
        }
      },
      ('<', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, kind: TokenKind::LTEQ};
        }
        else {
          t = Token{text: self.cur_char.to_string(), kind: TokenKind::LT};
        }
      },
      ('!', n) => {
        if '=' == n {
          let mut text = self.cur_char.to_string();
          self.next_char();
          text.push_str(&self.cur_char.to_string());
          t = Token{text: text, kind: TokenKind::NOTEQ};
        }
        else {
          t = Token{ text: self.cur_char.to_string(), kind: TokenKind::LEX_ERR}; // Unknown token
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

        t = Token{text: substring, kind: TokenKind::STRING};
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

        t = Token{text: substring, kind: TokenKind::NUMBER};
      },
      (c, _) if c.is_alphabetic() => {
        let mut substring = self.cur_char.to_string();

        while self.peek().is_alphabetic() {
          substring.push(self.peek());
          self.next_char();
        }
        
        let keyword = Token::check_if_keyword(&substring);
        match keyword {
          Some(k) => t = Token{text: substring, kind: k},
          None => t = Token{text: substring, kind: TokenKind::IDENT}
        }
      }
      _ => { 
        t = Token{text: self.cur_char.to_string(), kind: TokenKind::LEX_ERR}; // Unknown token
        let mut msg = String::from("Lexing error. ");
        msg.push_str(&self.cur_char.to_string());
        self.abort(msg);
      }
    }

    self.next_char(); // should refactor this side effect out nicely...
    t
  }
}

pub struct Token {
  pub text: String,
  pub kind: (&'static str, i16),
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
  text: String,
  val: i16,
}

impl TokenKind {
  pub const LEX_ERR: (&'static str, i16) = ("LEX_ERR", -2);
  pub const EOF: (&'static str, i16) = ("EOF", -1);
	pub const NEWLINE: (&'static str, i16) = ("NEWLINE", 0);
	pub const NUMBER: (&'static str, i16) = ("NUMBER", 1);
	pub const IDENT: (&'static str, i16) = ("IDENT", 2);
	pub const STRING: (&'static str, i16) = ("STRING", 3);
	// Keywords.
	pub const LABEL: (&'static str, i16) = ("LABEL", 101);
	pub const GOTO: (&'static str, i16) = ("GOTO", 102);
	pub const PRINT: (&'static str, i16) = ("PRINT", 103);
	pub const INPUT: (&'static str, i16) = ("INPUT", 104);
	pub const LET: (&'static str, i16) = ("LET", 105);
	pub const IF: (&'static str, i16) = ("IF", 106);
	pub const THEN: (&'static str, i16) = ("THEN", 107);
	pub const ENDIF: (&'static str, i16) = ("ENDIF", 108);
	pub const WHILE: (&'static str, i16) = ("WHILE", 109);
	pub const REPEAT: (&'static str, i16) = ("REPEAT", 110);
	pub const ENDWHILE: (&'static str, i16) = ("ENDWHILE", 111);
	// Operators.
	pub const EQ: (&'static str, i16) = ("EQ", 201);
	pub const PLUS: (&'static str, i16) = ("PLUS", 202);
	pub const MINUS: (&'static str, i16) = ("MINUS", 203);
	pub const ASTERISK: (&'static str, i16) = ("ASTERISK", 204);
	pub const SLASH: (&'static str, i16) = ("SLASH", 205);
	pub const EQEQ: (&'static str, i16) = ("EQEQ", 206);
	pub const NOTEQ: (&'static str, i16) = ("NOTEQ", 207);
	pub const LT: (&'static str, i16) = ("LT", 208);
	pub const LTEQ: (&'static str, i16) = ("LTEQ", 209);
	pub const GT: (&'static str, i16) = ("GT", 210);
	pub const GTEQ: (&'static str, i16) = ("GTEQ", 211);
  

  pub const ALL: [(&'static str, i16); 28] = [
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
