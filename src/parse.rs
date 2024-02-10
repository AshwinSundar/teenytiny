use std::process::exit;
use crate::lex::{Lexer, TokenType, TokenKind, Token};
use crate::emit::Emitter;

pub struct Parser<'a> {
  pub lexer: Lexer,
  pub emitter: &'a mut Emitter,
  pub cur_token: Token,
  pub peek_token: Token,
  pub symbols: Vec<String>, // Variables declared so far
  pub labels_declared: Vec<String>, // Labels declared so far
  pub labels_gotoed: Vec<String>, // Labels goto'ed so far
}

impl Parser<'_> {
  pub fn new(lx: Lexer, em: &mut Emitter) -> Parser {
    let mut p = Parser {
      lexer: lx,
      emitter: em,
      cur_token: Token { text: String::from(""), token_type: TokenKind::NONE },
      peek_token: Token { text: String::from(""), token_type: TokenKind::NONE },
      symbols: Vec::new(),
      labels_declared: Vec::new(),
      labels_gotoed: Vec::new(),
    }; 

    // Call twice to instantiate both current token and peek token
    p.next_token();
    p.next_token();
    p
  }

  // Return true if current token matches.
  pub fn check_token(&self, kind: &TokenType) -> bool {
    self.cur_token.token_type.0 == kind.0
  }

  // Return true if next token matches
  pub fn check_peek(&self, kind: TokenType) -> bool {
    self.peek_token.token_type.0 == kind.0
  }

  // Try to match current token, else error. Advances current token.
  pub fn match_token(&mut self, kind: &TokenType) {
    if !self.check_token(&kind) {
      let mut msg: String = String::from("Expected ");
      msg.push_str(kind.0);
      msg.push_str(", got ");
      msg.push_str(&self.cur_token.token_type.0);
      self.abort(msg);
    }

    self.next_token();
  }

  pub fn next_token(&mut self) {
    self.cur_token = self.peek_token.clone();
    self.peek_token = self.lexer.get_token().clone();
  }

  pub fn abort(&self, msg: String) {
    println!("{}", msg);
    exit(1)
  }

  pub fn program(&mut self) {
    println!("PROGRAM");
    self.emitter.header_line("#include <stdio.h>");
    self.emitter.header_line("int main(void){");

    // Skip excess newlines
    while self.check_token(&TokenKind::NEWLINE) {
      self.next_token();
    }

    while !self.check_token(&TokenKind::EOF) {
      self.statement();
    }

    // Close main function
    self.emitter.emit_line("return 0;");
    self.emitter.emit_line("}");

    for l in &self.labels_gotoed {
      if !self.labels_declared.contains(l) {
        let mut msg: String = String::from("Attempting to GOTO to undeclared label: ");
        msg.push_str(l);
        self.abort(msg);
      }
    }
  }

  pub fn statement(&mut self) {
    match self.cur_token.token_type {
      // "PRINT" (expression | string)
      TokenKind::PRINT => {
        self.next_token();

        if self.cur_token.token_type == TokenKind::STRING {
          // This is a simple string - just print it
          self.emitter.emit_line(&format!("printf(\"{});", &self.cur_token.text));
          self.emitter.emit_line("printf(\"\\n\");");

          self.next_token();
        }
        else {
          // This is an expression - evaluate it and print as a float
          self.emitter.emit_line(&format!("printf(\"%.2f\", (float)());"));
          self.expression();
        }
      }, 

      // "IF" comparison "THEN" nl { statement } "ENDIF" nl
      TokenKind::IF => {
        println!("STATEMENT-IF");
        self.next_token();
        self.comparison();

        self.match_token(&TokenKind::THEN);
        self.nl();

        while !self.check_token(&TokenKind::ENDIF) {
          self.statement();
        }

        self.match_token(&TokenKind::ENDIF);
      },

      // "WHILE" comparison "REPEAT" nl { statement nl} "ENDWHILE" nl
      TokenKind::WHILE => {
        println!("STATEMENT-WHILE");
        self.next_token();
        self.comparison();

        self.match_token(&TokenKind::REPEAT);
        self.nl();

        while !self.check_token(&TokenKind::ENDWHILE) {
          self.statement();
        }

        self.match_token(&TokenKind::ENDWHILE);
      },

      // "LABEL"
      TokenKind::LABEL => {
        println!("STATEMENT-LABEL");
        self.next_token();

        // Check if this label already exists (that means it's being declared twice, which is not allowed)
        if self.labels_declared.contains(&self.cur_token.text) {
          let mut msg: String = String::from("Label already exists: ");
          msg.push_str(&self.cur_token.text);
          self.abort(msg);
        }

        // Add this to list of labels that have been declared
        self.labels_declared.push(self.cur_token.text.clone());
        self.match_token(&TokenKind::IDENT);
      },
      
      // "GOTO"
      TokenKind::GOTO => {
        println!("STATEMENT-GOTO");
        self.next_token();
        self.labels_gotoed.push(self.cur_token.text.clone());
        self.match_token(&TokenKind::IDENT);
      },

      // "LET" ident "=" expression
      TokenKind::LET => {
        println!("STATEMENT-LET");
        self.next_token();

        if !self.symbols.contains(&self.cur_token.text) {
          self.symbols.push(self.cur_token.text.clone());
        }

        self.match_token(&TokenKind::IDENT);
        self.match_token(&TokenKind::EQ);
        self.expression();
      },

      // "INPUT" ident
      TokenKind::INPUT => {
        println!("STATEMENT-INPUT");
        self.next_token();

        if !self.symbols.contains(&self.cur_token.text) {
          self.symbols.push(self.cur_token.text.clone());
        }

        self.match_token(&TokenKind::IDENT);
      },
      _ => {
        let mut msg:String = String::from("Invalid statement at ");
        msg.push_str(&self.cur_token.text);
        msg.push_str(" (");
        msg.push_str(self.cur_token.token_type.0);
        msg.push_str(")");
        self.abort(msg);
      }
    }

    self.nl();

  }

  fn nl(&mut self) {
    println!("NEWLINE");

    // require at least one new line
    self.match_token(&TokenKind::NEWLINE);

    // but allow more than one as well
    while self.check_token(&TokenKind::NEWLINE) {
      self.next_token();
    }
  }

  // expression ::= term {( "-" | "+" ) term}
  fn expression(&mut self) {
    println!("EXPRESSION");
    self.term();

    // Can have 0 or more +/- and expressions.
    while self.check_token(&TokenKind::PLUS) || self.check_token(&TokenKind::MINUS) {
      self.next_token();
      self.term();
    }
  }

  fn comparison(&mut self) {
    println!("COMPARISON");
    self.expression();

    if self.is_comparison_operator() {
      self.next_token();
      self.expression();
    } else {
      let mut msg: String = String::from("Expected comparison operator at: ");
      msg.push_str(&self.cur_token.text);
      self.abort(msg);
    } 

    while self.is_comparison_operator() {
      self.next_token();
      self.expression();
    }
  }

  fn is_comparison_operator(&self) -> bool {
    self.check_token(&TokenKind::GT) 
    || self.check_token(&TokenKind::GTEQ)
    || self.check_token(&TokenKind::LT)
    || self.check_token(&TokenKind::LTEQ)
    || self.check_token(&TokenKind::EQEQ)
    || self.check_token(&TokenKind::NOTEQ)
  }

  // term ::= unary {( "/" | "*" ) unary}
  fn term(&mut self) {
    println!("TERM");
    self.unary();

    // Can have 0 or more *// and expressions
    while self.check_token(&TokenKind::ASTERISK) || self.check_token(&TokenKind::SLASH) {
      self.next_token();
      self.unary();
    }
  }

  // unary ::= ["+" | "-"] primary
  fn unary(&mut self) {
    println!("UNARY");

    // Optionary unary +/-
    if self.check_token(&TokenKind::PLUS) || self.check_token(&TokenKind::MINUS) {
      self.next_token();
    }

    self.primary();
  }

  // primary ::= number | ident
  fn primary(&mut self) {
    println!("PRIMARY ({})", &self.cur_token.text);

    if self.check_token(&TokenKind::NUMBER) {
      self.next_token();
    } 
    else if self.check_token(&TokenKind::IDENT) {
      // If variable doesn't already exist, then it's an error
      if !self.symbols.contains(&self.cur_token.text) {
        let mut msg: String = String::from("Referencing variable before assignment: ");
        msg.push_str(&self.cur_token.text);
        self.abort(msg);
      }

      self.next_token();
    } 
    else {
      let mut msg: String = String::from("Unexpected token at ");
      msg.push_str(&self.cur_token.text);
      self.abort(msg);
    }
  }
  

}