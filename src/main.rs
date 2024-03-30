mod city_data;

use city_data::CityData;

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::sync::{mpsc, Arc, Mutex, RwLock};
use std::thread;
use std::time::SystemTime;

fn main() {
    let start_time = SystemTime::now();
    let file = File::open("./measurements.txt").unwrap();
    let reader = io::BufReader::new(file);

    let cities_data = Arc::new(RwLock::new(BTreeMap::<String, Mutex<CityData>>::new()));
    let (tx, rx) = mpsc::channel::<String>();
    let rx = Arc::new(Mutex::new(rx));

    const NUM_THREADS: usize = 7;
    let mut threads = Vec::with_capacity(NUM_THREADS);
    for _ in 0..NUM_THREADS {
        let rx_thread = Arc::clone(&rx);
        let cities_data_thread = cities_data.clone();
        threads.push(thread::spawn(move || {
            loop {
                match rx_thread.lock().unwrap().recv() {
                    Ok(line) => {
                        if line.starts_with('#') {
                            continue;
                        }
                        if let Some((city, temperature)) = line.split_once(';') {
                            let temperature: f64 = temperature
                                .trim()
                                .parse()
                                .map_err(|_| format!("Can't parse {temperature}"))
                                .unwrap();

                            let cd_map = cities_data_thread.read().unwrap();
                            match cd_map.get(city) {
                                Some(e) => {
                                    e.lock().as_mut().unwrap().add(temperature);
                                }
                                None => {
                                    drop(cd_map);
                                    let mut cd_map = cities_data_thread.write().unwrap();
                                    cd_map
                                        .entry(city.into())
                                        .or_insert(Mutex::new(CityData::default()))
                                        .get_mut()
                                        .unwrap()
                                        .add(temperature);
                                }
                            }
                        }
                    }
                    Err(e) => {
                        println!("Error {e}");
                        break;
                    }
                }
            }
        }));
    }

    for line in reader.lines().flatten() {
        tx.send(line).unwrap();
    }
    drop(tx);
    for t in threads {
        t.join().unwrap();
    }

    let processing_duration = SystemTime::now().duration_since(start_time).unwrap();

    for (city, data) in cities_data.read().unwrap().iter() {
        let data = data.lock().unwrap();
        println!("{city}: {:.1}/{:.1}/{:.1}", data.min, data.mean(), data.max);
    }

    let total_duration = SystemTime::now().duration_since(start_time).unwrap();
    println!(
        "Processing duration: {} ms",
        processing_duration.as_millis()
    );
    println!("Total duration: {} ms", total_duration.as_millis());
}
