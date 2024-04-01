use options::{Mode, Options};

use std::time::SystemTime;

mod city_data;
mod multi_thread;
mod options;
mod single_thread;

fn main() {
    let options = Options::parse();

    let start_time = SystemTime::now();
    match options.mode {
        Mode::SingleThread => {
            single_thread::process_file(options.filename.as_str(), options.print_result)
        }
        Mode::Multithread => {
            multi_thread::process_file(options.filename.as_str(), options.print_result)
        }
    }
    let duration_ms = SystemTime::now()
        .duration_since(start_time)
        .unwrap()
        .as_millis();
    println!("Processing took {duration_ms} ms");
}
