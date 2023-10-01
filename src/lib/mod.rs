use anyhow::Result;
use std::fmt::Display;
use std::error::Error;

#[derive(Debug)]
pub enum ScanErr {
    TokenErr(&'static str)
}

impl Display for ScanErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ScanErr::TokenErr(msg) => f.write_str(msg)
        }
    }
}

impl Error for ScanErr {}

pub fn run(source: &str) -> Result<(), ScanErr> {
    println!("{}", source);
    Ok(())
}