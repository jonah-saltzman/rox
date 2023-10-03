mod object;
mod scanner;
mod token;
mod util;
mod error;
pub mod keyword;

use scanner::Scanner;
use error::{DisplayableVec, ParseError, RoxError};

pub struct Rox {}

impl Rox {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self, source: &str) -> Result<(), RoxError> {
        let mut scanner = Scanner::new(source);
        let tokens = scanner
            .scan()
            .map_err(|e| ParseError::ScanError(DisplayableVec(e)))?;
        for ref token in tokens {
            println!("{}", token);
        }
        Ok(())
    }
}
