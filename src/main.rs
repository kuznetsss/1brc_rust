mod city_data;

use city_data::CityData;

use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::str::from_utf8;
use std::thread;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let content = &fs::read("./measurements.txt").unwrap();

    const NUM_THREADS: usize = 8;
    let step = content.len() / NUM_THREADS;

    let result = thread::scope(|s| {
        let mut threads = Vec::with_capacity(NUM_THREADS);
        for i in 0..NUM_THREADS {
            let start = step * i;
            let end = if i != NUM_THREADS - 1 {
                start + step
            } else {
                content.len()
            };
            let content_ref = &content;
            println!("Processing {start} to {end}");
            threads.push(s.spawn(move || {
                process(content_ref, start, end)
            }));
        }

        let mut result_map = BTreeMap::<&str, CityData>::new();
        for t in threads {
            for (city, data) in t.join().unwrap() {
                result_map.entry(city).or_default().merge(&data);
            }
        }
        result_map

    });

    let processing_duration = SystemTime::now().duration_since(start_time).unwrap();


    for (city, data) in  result {
        println!("{city}: {:.1}/{:.1}/{:.1}", data.min, data.mean(), data.max);
    }

    let total_duration = SystemTime::now().duration_since(start_time).unwrap();
    println!(
        "Processing duration: {} ms",
        processing_duration.as_millis()
    );
    println!("Total duration: {} ms", total_duration.as_millis());
}

fn process<'a>(content: &'a [u8], start: usize, end: usize) -> HashMap<&str, CityData> {
    let mut cities_data = HashMap::<&'a str, CityData>::new();
    let start = to_end_of_line(start, content);
    let start = if content[start] == b'\n' {
        start + 1
    } else {
        start
    };
    let end = to_end_of_line(end, content);

    for line in content[start..end]
        .split(|c| c == &b'\n')
        .filter(|l| !l.is_empty())
    {
        process_line(from_utf8(line).unwrap(), &mut cities_data)
    }
    cities_data
}

fn to_end_of_line(mut ind: usize, content: &[u8]) -> usize {
    if ind == 0 {
        return ind;
    }

    while ind < content.len() && content[ind] != b'\n' {
        ind += 1;
    }
    ind
}

fn process_line<'a>(line: &'a str, cities_data: &mut HashMap<&'a str, CityData>) {
    let (city, temperature) = line
        .split_once(';')
        .ok_or(format!("Error on {line}"))
        .unwrap();
    let temperature: f64 = temperature.trim_end().parse().unwrap();
    cities_data.entry(city).or_default().add(temperature);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_end_of_line_test() {
        let content = "asdf\nhjkl\n".as_bytes();
        assert_eq!(to_end_of_line(0, content), 0);
        assert_eq!(to_end_of_line(1, content), 4);
        assert_eq!(to_end_of_line(4, content), 4);
        assert_eq!(to_end_of_line(6, content), 9);
    }

    #[test]
    fn process_line_test() {
        let mut cities_data = HashMap::<&str, CityData>::new();
        let line = &"asd;42.1";
        process_line(line, &mut cities_data);

        let data = cities_data.get("asd").unwrap();
        assert!((&data.min - 42.1).abs() <= 1e-9);
    }
}
