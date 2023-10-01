use super::object::Object;
use strum_macros::Display as S_Display;

#[derive(Debug, S_Display)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

#[derive(Debug)]
pub struct Token {
    typ: TokenType,
    lex: String,
    lit: Option<Object>,
    line: usize
}

impl Token {
    pub fn new(typ: TokenType, lex: String, lit: Option<Object>, line: usize) -> Self {
        Self {
            typ,
            lex: lex.to_owned(),
            lit,
            line,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.typ, self.lex)?;
        
        if let Some(ref lit) = self.lit {
            write!(f, " {}", lit)?;
        }
        
        write!(f, " {}", self.line)
    }
}
