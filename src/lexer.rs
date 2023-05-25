use logos::{Lexer, Logos};

#[derive(Logos, Debug, PartialEq, Copy, Clone)]
pub enum C1Token {
    #[token("bool")]
    KwBoolean,

    #[token("do")]
    KwDo,

    #[token("else")]
    KwElse,

    #[token("float")]
    KwFloat,

    #[token("for")]
    KwFor,

    #[token("if")]
    KwIf,

    #[token("int")]
    KwInt,

    #[token("printf")]
    KwPrintf,

    #[token("return")]
    KwReturn,

    #[token("void")]
    KwVoid,

    #[token("while")]
    KwWhile,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Asterisk,

    #[token("/")]
    Slash,

    #[token("=")]
    /// =
    Assign,

    #[token("==")]
    /// ==
    Equal,

    #[token("!=")]
    /// !=
    NotEqual,

    #[token("<")]
    /// <
    Less,

    #[token(">")]
    /// >
    Greater,

    #[token("<=")]
    /// <=
    LessEqual,

    #[token(">=")]
    /// >=
    GreaterEqual,

    #[token("&&")]
    /// &&
    And,

    #[token("||")]
    /// ||
    Or,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("(")]
    /// (
    LeftParenthesis,

    #[token(")")]
    /// )
    RightParenthesis,

    #[token("{")]
    /// {
    LeftBrace,

    #[token("}")]
    /// }
    RightBrace,

    #[regex("[0-9]+")]
    ConstInt,

    #[regex(r"(\d+\.\d+)|(\.\d+([eE]([-+])?\d+)?)|(\d+[eE]([-+])?\d+)")]
    ConstFloat,

    #[regex("true|false")]
    ConstBoolean,

    #[regex("\"[^\n\"]*\"")]
    ConstString,

    #[regex("[a-zA-Z]+[0-9a-zA-Z]*")]
    Identifier,

    #[regex(r"/\*[^\*/]*\*/", logos::skip)]
    CComment,

    #[regex("//[^\n]*(\n)?", logos::skip)]
    CPPComment,

    // We can also use this variant to define whitespace,
    // or any other matches we wish to skip.
    #[regex(r"[ \t\f\r]+", logos::skip)]
    Whitespace,

    #[regex(r"[\n]")]
    Linebreak,

    // Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    Error,
}

/// # Overview
/// Extended lexer based on the logos crate. The lexer keeps track of the current token and the next token
/// in the lexed text. Furthermore, the lexer keeps track of the line number in which each token is
/// located, and of the text associated with each token.
///
/// # Examples
/// ```
/// use cb_3::C1Lexer;
/// use cb_3::C1Token;
///     
/// let mut lexer = C1Lexer::new("void main() {
///                                 x = 4;
///                               }");
/// assert_eq!(lexer.current_token(), Some(C1Token::KwVoid));
/// assert_eq!(lexer.current_line_number(), Some(1));
/// assert_eq!(lexer.peek_token(), Some(C1Token::Identifier));
/// assert_eq!(lexer.peek_line_number(), Some(1));
///
/// lexer.eat();
/// // current token is 'main'
///
/// lexer.eat();
/// lexer.eat();
/// lexer.eat();
/// // current token is '{'
///
/// assert_eq!(lexer.current_token(), Some(C1Token::LeftBrace));
/// assert_eq!(lexer.current_line_number(), Some(1));
///
/// // next token is 'x'
/// assert_eq!(lexer.peek_token(), Some(C1Token::Identifier));
/// assert_eq!(lexer.peek_text(), Some("x"));
/// assert_eq!(lexer.peek_line_number(), Some(2));
/// ```
pub struct C1Lexer<'a> {
    logos_lexer: Lexer<'a, C1Token>,
    logos_line_number: usize,
    current_token: Option<TokenData<'a>>,
    peek_token: Option<TokenData<'a>>,
}

impl<'a> C1Lexer<'a> {
    /// Initialize a new C1Lexer for the given string slice
    pub fn new(text: &'a str) -> C1Lexer {
        let mut lexer = C1Lexer {
            logos_lexer: C1Token::lexer(text),
            logos_line_number: 1,
            current_token: None,
            peek_token: None,
        };
        lexer.current_token = lexer.next_token();
        lexer.peek_token = lexer.next_token();
        lexer
    }

    /// Return the C1Token variant of the current token without consuming it.
    /// ```
    /// use cb_3::{C1Lexer, C1Token};
    /// let lexer = C1Lexer::new("current next");
    ///
    /// assert_eq!(lexer.current_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.current_text(), Some("current"));
    ///
    /// assert_eq!(lexer.current_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.current_text(), Some("current"));
    /// ```
    pub fn current_token(&self) -> Option<C1Token> {
        self.current_token.token_type()
    }

    /// Return the C1Token variant of the next token without consuming it.
    ///```
    /// use cb_3::{C1Lexer, C1Token};
    /// let lexer = C1Lexer::new("current next");
    ///
    /// assert_eq!(lexer.peek_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.peek_text(), Some("next"));
    ///
    /// assert_eq!(lexer.peek_token(), Some(C1Token::Identifier));
    /// assert_eq!(lexer.peek_text(), Some("next"));
    /// ```
    pub fn peek_token(&self) -> Option<C1Token> {
        self.peek_token.token_type()
    }

    /// Return the text of the current token
    pub fn current_text(&self) -> Option<&str> {
        self.current_token.text()
    }

    /// Return the text of the next token
    pub fn peek_text(&self) -> Option<&str> {
        self.peek_token.text()
    }

    /// Return the line number where the current token is located
    pub fn current_line_number(&self) -> Option<usize> {
        self.current_token.line_number()
    }

    /// Return the line number where the next token is located
    pub fn peek_line_number(&self) -> Option<usize> {
        self.peek_token.line_number()
    }

    /// Drop the current token and retrieve the next token in the text.
    /// ```
    /// use cb_3::{C1Lexer, C1Token};
    /// let mut lexer = C1Lexer::new("current next last");
    ///
    /// assert_eq!(lexer.current_text(), Some("current"));
    /// assert_eq!(lexer.peek_text(), Some("next"));
    ///
    /// lexer.eat();
    /// assert_eq!(lexer.current_text(), Some("next"));
    /// assert_eq!(lexer.peek_text(), Some("last"));
    ///
    /// lexer.eat();
    /// assert_eq!(lexer.current_text(), Some("last"));
    /// assert_eq!(lexer.peek_text(), None);
    ///
    /// lexer.eat();
    /// assert_eq!(lexer.current_text(), None);
    /// assert_eq!(lexer.peek_text(), None);
    /// ```
    pub fn eat(&mut self) {
        self.current_token = self.peek_token.take();
        self.peek_token = self.next_token();
    }

    /// Private method for reading the next token from the logos::Lexer and extracting the required data
    /// from it
    fn next_token(&mut self) -> Option<TokenData<'a>> {
        // Retrieve the next token from the internal lexer
        if let Some(c1_token) = self.logos_lexer.next() {
            match c1_token {
                C1Token::Linebreak => {
                    // If the token is a linebreak, increase the line number and get the next token
                    self.logos_line_number += 1;
                    self.next_token()
                }
                _ => Some(TokenData {
                    // If the token is not a linebreak, initialize and return a TokenData instance
                    token_type: c1_token,
                    token_text: self.logos_lexer.slice(),
                    token_line: self.logos_line_number,
                }),
            }
        } else {
            None
        }
    }
}

/// Hidden struct for capsuling the data associated with a token.
struct TokenData<'a> {
    token_type: C1Token,
    token_text: &'a str,
    token_line: usize,
}

/// Hidden trait that makes it possible to implemented the required getter functionality directly for
/// Option<TokenData>.
trait TokenDataProvider<'a> {
    /// Return the type of the token, aka. its C1Token variant.
    fn token_type(&self) -> Option<C1Token>;
    /// Return the text of the token
    fn text(&self) -> Option<&str>;
    /// Return the line number of the token
    fn line_number(&self) -> Option<usize>;
}

impl<'a> TokenDataProvider<'a> for Option<TokenData<'a>> {
    fn token_type(&self) -> Option<C1Token> {
        self.as_ref().map(|data| data.token_type)
    }

    fn text(&self) -> Option<&'a str> {
        self.as_ref().map(|data| data.token_text)
    }

    fn line_number(&self) -> Option<usize> {
        self.as_ref().map(|data| data.token_line)
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::C1Lexer;
    use crate::C1Token;

    #[test]
    fn lines_are_counted() {
        let mut lexer1 = C1Lexer::new("Hello\nTest");
        assert_eq!(lexer1.current_line_number(), Some(1));
        assert_eq!(lexer1.peek_line_number(), Some(2));
        lexer1.eat();
        assert_eq!(lexer1.current_line_number(), Some(2));
        assert_eq!(lexer1.peek_line_number(), None);
        lexer1.eat();
        assert_eq!(lexer1.current_line_number(), None);
        assert_eq!(lexer1.peek_line_number(), None);
    }

    #[test]
    fn line_count_is_reset() {
        {
            let mut lexer1 = C1Lexer::new("Hello\nTest\nbla\nfoo");
            lexer1.eat();
            lexer1.eat();
            assert_eq!(lexer1.current_line_number(), Some(3));
            assert_eq!(lexer1.peek_line_number(), Some(4));
        }
        let lexer2 = C1Lexer::new("bool foo()");
        assert_eq!(lexer2.current_line_number(), Some(1));
        assert_eq!(lexer2.peek_line_number(), Some(1));
    }

    #[test]
    fn float_recognition() {
        let lexer = C1Lexer::new("1.2");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.000");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new(".2");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2e4");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2e+4");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2e-10");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("1.2E-10");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));

        let lexer = C1Lexer::new("33E+2");
        assert_eq!(lexer.current_token(), Some(C1Token::ConstFloat));
    }
}
