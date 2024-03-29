mod city_data;

use city_data::CityData;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let file = File::open("./measurements.txt").unwrap();
    let mut reader = io::BufReader::new(file);

    let mut cities_data = BTreeMap::<String, CityData>::new();

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
    let processing_duration = SystemTime::now().duration_since(start_time).unwrap();

    for (city, data) in cities_data.iter() {
        println!("{city}: {:.1}/{:.1}/{:.1}", data.min, data.mean(), data.max);
    }

    let total_duration = SystemTime::now().duration_since(start_time).unwrap();
    println!(
        "Processing duration: {} ms",
        processing_duration.as_millis()
    );
    println!("Total duration: {} ms", total_duration.as_millis());
}
