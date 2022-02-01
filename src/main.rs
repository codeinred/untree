use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Error as ioError, Lines, Stdin};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => process_lines("(standard input)", read_stdin()),
        2 => match args[1].as_ref() {
            "-" => process_lines("(standard input)", read_stdin()),
            "-h" | "--help" => print_help(args),
            filename => match read_lines(filename) {
                Ok(lines) => process_lines(filename, lines),
                Err(err) => println!("Unable to open '{filename}': {err}"),
            },
        },
        _ => print_help(args),
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
fn read_lines<P: AsRef<Path>>(filename: P) -> Result<Lines<BufReader<File>>, ioError>
{
    File::open(filename).map(|file| io::BufReader::new(file).lines())
}

fn process_lines(filename: &str, lines: Lines<impl BufRead>) {
    println!("      │ File: {filename}");
    println!("──────┼────────────────────────────────");
    for (i, result) in lines.enumerate() {
        match result {
            Ok(line) => println!("{i:5} │ {line}"),
            Err(err) => println!("Error: {err}"),
        }
    }
}
