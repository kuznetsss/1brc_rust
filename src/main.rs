mod city_data;

use city_data::CityData;

use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let content = fs::read("./measurements.txt").unwrap();
    let cities_data = Arc::new(RwLock::new(HashMap::<&str, Mutex<CityData>>::new()));

    const NUM_THREADS: usize = 8;
    let mut threads = Vec::with_capacity(NUM_THREADS);
    let step = content.len() / NUM_THREADS;
    for i in 0..NUM_THREADS {
        threads.push(thread::scope(|s| {
            s.spawn(|| {
                let start = step * i;
                let end = start + step;
                process(&content, start, end, cities_data);
            })
        }));
    }
    for t in threads {
        t.join().unwrap();
    }

    let processing_duration = SystemTime::now().duration_since(start_time).unwrap();

    let cities_data_read = cities_data.read().unwrap();
    let mut cities_sorted: Vec<_> = cities_data_read.keys().collect();
    cities_sorted.sort_unstable();

    for city in cities_sorted {
        let data = cities_data_read.get(city).unwrap().lock().unwrap();
        println!("{city}: {:.1}/{:.1}/{:.1}", data.min, data.mean(), data.max);
    }

    let total_duration = SystemTime::now().duration_since(start_time).unwrap();
    println!(
        "Processing duration: {} ms",
        processing_duration.as_millis()
    );
    println!("Total duration: {} ms", total_duration.as_millis());
}

type CitiesData<'a> = Arc<RwLock<HashMap<&'a str, Mutex<CityData>>>>;

fn process<'a>(content: &'a Vec<u8>, start: usize, end: usize, cities_data: CitiesData<'a>) {
    let mut it = start;
    while it < end && content[it] != b'\n' {
        it += 1;
    }
    it += 1;
    let mut line_start = it;
    while it < end {
        line_start = it;
        while it < end && content[it] != b'\n' {
            it += 1;
        }
        if it >= end {
            break;
        }
        let line = std::str::from_utf8(&content[line_start..it]).unwrap();
        process_line(line, &cities_data);
        it += 1;
    }
    while it < content.len() && content[it] != b'\n' {
        it += 1;
    }
    let line = std::str::from_utf8(&content[line_start..it]).unwrap();
    process_line(line, &cities_data);
}

fn process_line<'a>(line: &'a str, cities_data: &CitiesData<'a>) {
    let (city, temperature) = line.split_once(';').unwrap();
    let temperature: f64 = temperature.trim_end().parse().unwrap();
    let cities_data_read = cities_data.read().unwrap();
    match cities_data_read.get(city) {
        Some(e) => e.lock().unwrap().add(temperature),
        None => {
            drop(cities_data_read);
            let mut new_data = CityData::default();
            new_data.add(temperature);
            cities_data
                .write()
                .unwrap()
                .insert(city, Mutex::new(new_data));
        }
    }
}
