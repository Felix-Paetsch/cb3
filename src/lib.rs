mod lexer;

// Type definition for the Result that is being used by the parser. You may change it to anything
// you want
pub type ParseResult = Result<(), String>;

pub use lexer::C1Lexer;
pub use lexer::C1Token;

// You will need a re-export of your C1Parser definition. Here is an example:
mod parser;
pub use parser::C1Parser;
