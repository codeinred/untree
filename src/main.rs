use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let program_name = &args[0];
        println!("Usage:\n\t{program_name} <filename>")
    } else {
        let filename = &args[1];
        match read_lines(filename) {
            Ok(lines) => {
                println!("      │ File: {filename}");
                println!("──────┼────────────────────────────────");
                for (i, result) in lines.enumerate() {
                    match result {
                        Ok(line) => println!("{i:5} │ {line}"),
                        Err(err) => println!("Error: {err}"),
                    }
                }
            }
            Err(err) => {
                println!("Unable to open '{filename}': {err}")
            }
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
