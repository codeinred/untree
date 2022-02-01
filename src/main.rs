use std::env;
use std::fs::File;
use std::io::{self, BufRead, Stdin};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => process_lines("(standard input)", read_stdin()),
        2 => match args[1].as_ref() {
            "-" => process_lines("(standard input)", read_stdin()),
            filename => match read_lines(filename) {
                Ok(lines) => process_lines(filename, lines),
                Err(err) => println!("Unable to open '{filename}': {err}"),
            },
        },
        num_inputs => {
            let program_name = if num_inputs == 0 {
                "<unknown>"
            } else {
                &args[0]
            };
            println!("Usage:\n\t{program_name} [filename]");
            println!();
            println!("If a filename is provided, reads lines from that file. Otherwise, reads lines from stdin");
        }
    }
}

fn read_stdin() -> io::Lines<io::BufReader<Stdin>> {
    io::BufReader::new(io::stdin()).lines()
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P: AsRef<Path>>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn process_lines<T: io::BufRead>(filename: &str, lines: io::Lines<T>) {
    println!("      │ File: {filename}");
    println!("──────┼────────────────────────────────");
    for (i, result) in lines.enumerate() {
        match result {
            Ok(line) => println!("{i:5} │ {line}"),
            Err(err) => println!("Error: {err}"),
        }
    }
}
