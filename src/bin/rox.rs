use anyhow::{Context, Result};
use std::io::{self, Write, BufRead};
use rox::Rox;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rox = Rox::new();
    match args.len() {
        0 => run_prompt(rox),
        1 => run_file(rox, &args[0]),
        _ => {
            println!("Usage: rox [script]");
            std::process::exit(64)
        }
    }
}

fn run_prompt(mut rox: Rox) -> Result<()> {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        print!("> ");

        io::stdout().flush().context("failed to flush stdout")?;

        rox.run(&line.context("failed to read line")?).context("failed to run line")?
    }
    Ok(())
}

fn run_file(mut rox: Rox, path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path).context(format!("failed to open {}", path))?;
    rox.run(&file).context(format!("failed to run {}", path))
}
