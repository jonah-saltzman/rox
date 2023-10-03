use anyhow::{Context, Result};
use std::io::{self, Write};
use rox_lib::Rox;

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

    loop {
        print!("> ");
        io::stdout().flush().context("Failed to flush stdout")?;

        let mut input = String::new();
        stdin.read_line(&mut input).context("Failed to read line")?;

        if input.trim().is_empty() {
            continue;
        }

        match rox.run(&input) {
            Err(e) => { println!("{}", e) },
            _ => {}
        }
    }

}

fn run_file(mut rox: Rox, path: &str) -> Result<()> {
    let file = std::fs::read_to_string(path).context(format!("failed to open {}", path))?;
    rox.run(&file).context(format!("failed to run {}", path))
}
