use crate::object::Object;

use super::object;
use super::token::*;
use super::error::ScanErr;

pub struct Scanner {
    tokens: Vec<Token>,
    errors: Vec<ScanErr>,
    src: Vec<char>,
    start: usize,
    curr: usize,
    line: usize,
}

impl Scanner {
    pub fn new(src: &str) -> Self {
        Self {
            src: src.chars().collect(),
            tokens: vec![],
            errors: vec![],
            start: 0,
            curr: 0,
            line: 1,
        }
    }

    pub fn scan(&mut self) -> Result<Vec<Token>, Vec<ScanErr>> {
        while self.curr < self.src.len() {
            self.start = self.curr;
            match *self.advance() {
                '(' => self.add_token(TokenType::LeftParen, None),
                ')' => self.add_token(TokenType::RightParen, None),
                '{' => self.add_token(TokenType::LeftBrace, None),
                '}' => self.add_token(TokenType::RightBrace, None),
                ',' => self.add_token(TokenType::Comma, None),
                '.' => self.add_token(TokenType::Dot, None),
                '-' => self.add_token(TokenType::Minus, None),
                '+' => self.add_token(TokenType::Plus, None),
                ';' => self.add_token(TokenType::Semicolon, None),
                '*' => self.add_token(TokenType::Star, None),
                c => self.err(ScanErr::UnexpectedToken(format!("unexpected token {}", c)))
            }
        }
        if self.errors.len() > 0 {
            Err(core::mem::take(&mut self.errors))
        } else {
            Ok(core::mem::take(&mut self.tokens))
        }
    }

    fn add_token(&mut self, typ: TokenType, lit: Option<Object>) {
        let lex = self.src[self.start..self.curr].iter().collect();
        self.tokens.push(Token::new(typ, lex, lit, self.line));
    }

    fn err(&mut self, err: ScanErr) {
        self.errors.push(err)
    }

    fn at_end(&self) -> bool {
        return self.curr >= self.src.len();
    }

    fn advance(&mut self) -> &char {
        let c = self.src
            .get(self.curr)
            .expect("invalid advance call");
        self.curr += 1;
        c
    }

    fn peek(&self) -> Option<&char> {
        self.src.get(self.curr + 1)
    }

    fn peek_two(&self) -> Option<&char> {
        self.src.get(self.curr + 2)
    }
}
