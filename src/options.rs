use std::{env, process::exit};

pub enum PrintResult {
    Yes,
    No,
}

pub enum Mode {
    SingleThread,
    Multithread,
}

pub struct Options {
    pub filename: String,
    pub mode: Mode,
    pub print_result: PrintResult,
}

impl Options {
    pub fn parse() -> Self {
        let mut options = Options {
            filename: "./measurements.txt".to_string(),
            mode: Mode::Multithread,
            print_result: PrintResult::Yes,
        };
        for opt in env::args().skip(1) {
            match opt.as_str() {
                "-s" => options.mode = Mode::SingleThread,
                "-m" => options.mode = Mode::Multithread,
                "-q" => options.print_result = PrintResult::No,
                "-h" | "--help" => {
                    print_help();
                    exit(0);
                },
                s => match s.chars().next().unwrap() {
                    '-' => {
                        eprintln!("Unknown option {s}");
                        print_help();
                        exit(1);
                    }
                    _ => {
                        options.filename = s.to_string();
                    }
                },
            }
        }
        options
    }
}

fn print_help() {
    println!(
        "
Usage: brc [-s | -m] [-q] [filename]
Options:
    -s - run in a single thread mode
    -m - run in a multithread mode (default mode)
    -q - don't print the results
    filename - file to read data from (default: ./measurements.txt)
"
    );
}
