use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::SystemTime;

use crate::city_data::CityData;
use crate::options::PrintResult;

pub fn process_file(filepath: &str, print_result: PrintResult) {
    let file = File::open("./measurements.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut cities_data = HashMap::<String, CityData>::new();

    let mut line = String::with_capacity(1024);

    while reader.read_line(&mut line).unwrap() > 0 {
        if line.starts_with('#') {
            line.clear();
            continue;
        }
        if let Some((city, temperature)) = line.split_once(';') {
            let temperature: f64 = temperature
                .trim()
                .parse()
                .map_err(|_| format!("Can't parse {temperature}"))
                .unwrap();
            cities_data
                .entry(city.to_string())
                .or_default()
                .add(temperature);
        }
        line.clear()
    }

    let mut cities_sorted: Vec<_> = cities_data.keys().collect();
    cities_sorted.sort_unstable();

    if matches!(print_result, PrintResult::Yes) {
        for city in cities_sorted {
            let data = cities_data.get(city).unwrap();
            println!("{city}: {:.1}/{:.1}/{:.1}", data.min, data.mean(), data.max);
        }
    }
}
