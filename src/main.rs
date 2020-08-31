use rustyline::{Editor, error::ReadlineError};

fn main() -> Result<(), anyhow::Error> {
    // `()` can be used when no completer is required
    let mut reader = Editor::<()>::new();

    loop {
        match reader.readline("> ") {
            Ok(input) => {
                reader.add_history_entry(&input);
                execute_input(&input)?;
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
    println!("Got input: {}", input);
    Ok(())
}
