mod day1;
mod day2;

use clap::Parser;
use std::fs::read_to_string;
use std::path::PathBuf;

/// 2023 Advent of code in Rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// path to input file
    #[arg(short, long)]
    input: PathBuf,

    /// which day to run
    #[arg(short, long)]
    day: u16,
}

fn main() {
    let args = Args::parse();

    println!("Processing day {}:", args.day);

    match read_to_string(args.input) {
        Ok(file_as_string) => {
            let result = match args.day {
                1 => day1::process(file_as_string),
                2 => day2::process(file_as_string),
                _ => {
                    panic!("bad day input, or day not completed");
                }
            };

            println!("result: {}", result);
        }
        Err(err) => {
            println!("Failure reading file: {}", err.to_string())
        }
    }
}
