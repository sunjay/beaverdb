mod language;
mod storage;

use anyhow::Context;
use rustyline::{Editor, error::ReadlineError};

use crate::language::Stmt;

fn main() -> Result<(), anyhow::Error> {
    // `()` can be used when no completer is required
    let mut reader = Editor::<()>::new();

    loop {
        match reader.readline("> ") {
            Ok(line) => {
                reader.add_history_entry(&line);
                println!("Line: {}", line);
            },

            Err(ReadlineError::Interrupted) |
            Err(ReadlineError::Eof) => {
                break
            },

            Err(err) => Err(err)?,
        }
    }

    Ok(())
}

fn execute_input(input: &str) -> Result<(), anyhow::Error> {
    let query: Stmt = input.parse()
        .context("Unable to parse input")?;

    Ok(())
}
