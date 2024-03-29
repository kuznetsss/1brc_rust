mod city_data;

use city_data::CityData;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let file = File::open("./measurements.txt").unwrap();
    let reader = io::BufReader::new(file);

    let mut cities_data = BTreeMap::<String, CityData>::new();

    for line in reader.lines().flatten().take(1000) {
        if line.starts_with('#') {
            continue;
        }
        if let Some((city, temperature)) = line.split_once(';') {
            let temperature: f64 = temperature.parse().unwrap();
            cities_data.entry(city.to_string()).or_default().add(temperature);
        }
    }
    let processing_duration = SystemTime::now().duration_since(start_time).unwrap();

    for (city, data) in cities_data.iter() {
        println!("{city}: {:.1}/{:.1}/{:.1}", data.min, data.mean(), data.max);
    }

    let total_duration = SystemTime::now().duration_since(start_time).unwrap();
    println!("Processing duration: {} ms", processing_duration.as_millis());
    println!("Total duration: {} ms", total_duration.as_millis());
}
