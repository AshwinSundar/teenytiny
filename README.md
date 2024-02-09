# Teeny Tiny Compiler

This is a compiler for a dialect of [BASIC](https://en.wikipedia.org/wiki/BASIC) called `Teeny Tiny`. It is a Rust port of Austin Henley's [implementation in Python](https://austinhenley.com/blog/teenytinycompiler1.html). One day, if I'm brave enough, I'll write my own tutorial as well.  

## Definitions

**Lexer**: Using input code, generates bite-sized components called "tokens"

**Parser**: Takes the output of the lexer (i.e. the tokens) and verifies that the tokens occur in an order that is allowed

**Emitter**: Converts valid sequences of tokens into code in the target language (in this case, `C`)  