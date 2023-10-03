use crate::error::ScanErr;
use crate::object::Object;
use crate::token::*;

// include!(concat!(env!("OUT_DIR"), "/keyword_map.rs"));

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
            match self.advance() {
                Some('(') => self.add_token(TokenType::LeftParen),
                Some(')') => self.add_token(TokenType::RightParen),
                Some('{') => self.add_token(TokenType::LeftBrace),
                Some('}') => self.add_token(TokenType::RightBrace),
                Some(',') => self.add_token(TokenType::Comma),
                Some('.') => self.add_token(TokenType::Dot),
                Some('-') => self.add_token(TokenType::Minus),
                Some('+') => self.add_token(TokenType::Plus),
                Some(';') => self.add_token(TokenType::Semicolon),
                Some('*') => self.add_token(TokenType::Star),
                Some('\n') => self.line += 1,
                Some(&c) if c.is_whitespace() => {}
                Some('!') => match self.match_next('=') {
                    true => self.add_token(TokenType::BangEqual),
                    false => self.add_token(TokenType::Bang),
                },
                Some('=') => match self.match_next('=') {
                    true => self.add_token(TokenType::EqualEqual),
                    false => self.add_token(TokenType::Equal),
                },
                Some('>') => match self.match_next('=') {
                    true => self.add_token(TokenType::GreaterEqual),
                    false => self.add_token(TokenType::Greater),
                },
                Some('<') => match self.match_next('=') {
                    true => self.add_token(TokenType::LessEqual),
                    false => self.add_token(TokenType::Less),
                },
                Some('/') => match self.match_next('/') {
                    true => {
                        while let Some(&c) = self.peek() {
                            if c == '\n' {
                                self.line += 1;
                                break;
                            }
                            self.advance();
                        }
                    }
                    false => self.add_token(TokenType::Slash),
                },
                Some('"') => {
                    let mut ended = false;
                    while let Some(&c) = self.peek() {
                        self.advance();
                        if c == '\n' {
                            self.line += 1
                        }
                        if c == '"' {
                            self.add_token(TokenType::String);
                            ended = true;
                            break;
                        }
                    }
                    if !ended {
                        self.err(ScanErr::UnexpectedEOF(
                            "expected '\"' before EOF".to_owned(),
                        ))
                    }
                }
                Some('0'..='9') => {
                    let mut saw_dot = false;
                    let mut well_formed = true;
                    while let Some(&c) = self.peek() {
                        match c {
                            '0'..='9' => {}
                            '.' => match self.peek_next() {
                                Some('0'..='9') => {
                                    if saw_dot {
                                        well_formed = false;
                                    }
                                    saw_dot = true;
                                }
                                _ => {
                                    well_formed = false;
                                }
                            },
                            c if c.is_whitespace() => {
                                break;
                            }
                            _ => well_formed = false,
                        }
                        self.advance();
                    }
                    if well_formed {
                        self.add_token(TokenType::Number)
                    } else {
                        self.err(ScanErr::ParseNumErr("improperly formed number".to_owned()))
                    }
                }
                Some(&c) if c.is_alphabetic() => {
                    while let Some(&c) = self.peek() {
                        self.advance();
                        if !c.is_alphabetic() {
                            break;
                        }
                    }
                    self.identifier()
                }
                Some(&c) => self.err(ScanErr::UnexpectedToken(format!("unexpected token {}", c))),
                None => self.add_token(TokenType::EOF),
            }
        }
        if self.errors.len() > 0 {
            Err(core::mem::take(&mut self.errors))
        } else {
            Ok(core::mem::take(&mut self.tokens))
        }
    }

    fn identifier(&mut self) {
        // let lex = self.get_current_lex();
    }

    fn add_token(&mut self, typ: TokenType) {
        let lex = self.get_current_lex();
        let lit: Option<Object> = match typ {
            TokenType::Number => Some(Object::Number(
                lex.parse().expect("already checked for valid number"),
            )),
            TokenType::String => Some(Object::String(lex[1..lex.len() - 1].to_owned())),
            _ => None,
        };
        self.tokens.push(Token::new(typ, lex, lit, self.line));
    }

    fn get_current_lex(&self) -> String {
        self.src[self.start..self.curr].iter().collect()
    }

    fn err(&mut self, err: ScanErr) {
        self.errors.push(err)
    }

    fn match_next(&mut self, target: char) -> bool {
        match self.peek() {
            Some(&c) if c == target => {
                self.curr += 1;
                true
            }
            _ => false,
        }
    }

    fn advance(&mut self) -> Option<&char> {
        if let Some(c) = self.src.get(self.curr) {
            self.curr += 1;
            Some(c)
        } else {
            None
        }
    }

    fn peek(&self) -> Option<&char> {
        self.src.get(self.curr)
    }

    fn peek_next(&self) -> Option<&char> {
        self.src.get(self.curr + 1)
    }
}
