use anyhow::{Context, Result};
use std::io::{self, Write, BufRead};
use rox::run;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.len() {
        0 => run_prompt(),
        1 => run_file(&args[0]),
        _ => {
            println!("Usage: rox [script]");
            std::process::exit(64)
        }
    }
}

fn run_prompt() -> Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        print!("> ");

        io::stdout().flush().context("failed to flush stdout")?;

        run(&line.context("failed to read line")?).context("failed to run line")?
    }
    Ok(())
}

fn run_file(path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path).context(format!("failed to open {}", path))?;
    run(&file).context(format!("failed to run {}", path))
}
