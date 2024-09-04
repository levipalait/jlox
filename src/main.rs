use std::io::Write;
use anyhow::Result;

use scanner::Scanner;
use crate::errors::*;

mod scanner;
mod errors;
mod token_type;
mod token;
mod literal;

/// Takes in command line arguments and decides whether to run 
/// jlox on a source file or to open the prompt mode. If nothing
/// matches, it will return an Error with the desired message.
/// Also, if the code execution fails, an Error is returned.
fn main() -> Result<()> {
    // Retreive command line arguments
    let argv: Vec<String> = std::env::args().collect();
    let argc: usize = argv.len();

    // Check argument vector length to either run a script
    // from a source file or run the prompt mode of jlox
    if argc > 2 {
        Err(ArgumentError::InvalidArgs.into())// Return an error if argc doesn't match desired length
    } else if argc == 2 {
        let file_path = argv.get(1).ok_or(ArgumentError::ArgAccessError)?.to_owned();
        run_file(file_path) // Return the Result of the run_file function
    } else {
        run_prompt() // Return the Result of the run_prompt function
    }
}

/// Takes in a file path as a `String`, loads the file content
/// into memory as another `String` and runs the source code
/// by calling [run]
fn run_file(file_path: String) -> Result<()> {
    let source = std::fs::read_to_string(file_path)?;
    run(source) // Return the Result of the run function
}

/// Runs the prompt mode of jlox. It takes in user input from the
/// cli and runs the given source code by calling [run]
fn run_prompt() -> Result<()> {
    loop {
        print!("> ");
        std::io::stdout().flush()?; // Print '> ' to the cli

        let mut line = String::new();
        std::io::stdin().read_line(&mut line)?; // Read cli input into a String

        if line.trim().is_empty() {
            break Ok(()); // If no input was given, the prompt mode is exited with an Ok
        }

        run(line)?; // Run the source code given by the cli
    }
}

/// Takes in Lox source code as a `String` and starts the running
/// process on it.
fn run(source: String) -> Result<()> {
    
    let scanner = Scanner::new(source);
    let tokens = scanner.scan_tokens()?;

    for token in tokens {
        println!("{}", token);
    }

    Ok(())
}