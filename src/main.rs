use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines, Stdin};
use std::path::Path;
use std::vec::Vec;

use colored::*;

mod macros;
use macros::either;

type IO = Result<(), io::Error>;
type IOResult<T> = Result<T, io::Error>;

fn main() -> IO {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => process_lines("(standard input)", read_stdin()),
        2 => match args[1].as_ref() {
            "-" => process_lines("(standard input)", read_stdin()),
            "-h" | "--help" => Ok(print_help(args)),
            filename => process_lines(filename, read_lines(filename)?),
        },
        _ => Ok(print_help(args)),
    }
}

fn print_help(args: Vec<String>) {
    let program_name = if args.len() == 0 {
        "<unknown>"
    } else {
        &args[0]
    };
    println!();
    println!("Usage:");
    println!();
    println!("    {program_name} [<filename>]");
    println!();
    println!("Reads lines from standard input, unless a filename is provided.");
    println!("If the filename is '-', standard input is used as the file.");
}

fn read_stdin() -> Lines<BufReader<Stdin>> {
    io::BufReader::new(io::stdin()).lines()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines(file: impl AsRef<Path>) -> IOResult<Lines<BufReader<File>>> {
    File::open(file).map(|file| io::BufReader::new(file).lines())
}

/**
 * Returns an entry in the tree, where the first result is the depth, and the second result is the file
 */
fn get_entry(entry: &str) -> (i32, &str) {
    match either!(
        entry.strip_prefix("    "),
        entry.strip_prefix("└── "),
        entry.strip_prefix("├── "),
        entry.strip_prefix("│   ")
    ) {
        Some(suffix) => {
            let (i, result) = get_entry(suffix);
            (i + 1, result)
        }
        None => (0, entry),
    }
}

fn process_lines(_filename: &str, lines: Lines<impl BufRead>) -> IO {
    // let mut pathStack = vec!("");

    for result in lines {
        let line = result?;
        if line == "" {
            // We're done
            return Ok(());
        }

        let (depth, filename) = get_entry(line.as_ref());
        println!(
            "depth={}, filename={}",
            depth.to_string().bold(),
            filename.blue().bold()
        );
    }

    Ok(())
}
