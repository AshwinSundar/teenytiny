use std::{fs::File, io::Write};

pub struct Emitter {
  pub full_path: String, // file path to write C code to
  pub code: String, // C code that will eventually be output
  pub header: String,
}

impl Emitter {
  pub fn new(full_path: String) -> Emitter {
    Emitter {
      full_path: full_path,
      code: String::new(),
      header: String::new(),
    }
  }

  pub fn emit(&mut self, code: &str) {
    self.code.push_str(code);
  }

  pub fn emit_line(&mut self, code: &str) {
    self.code.push_str(code);
    self.code.push_str("\n"); // Questionable
  }

  pub fn header_line(&mut self, code: &str) {
    self.header.push_str(code);
    self.header.push_str("\n"); // Questionable
  }

  pub fn write_file(&self) {
    let mut file:File = File::create(&self.full_path).unwrap(); // Change to unwrap_or_else, with abort message
    let mut contents: String = self.header.clone();
    contents.push_str(&self.code);
    file.write_all(contents.as_bytes()).unwrap(); // Change to unwrap_or_else, with abort message
  }
}