use chrono::prelude::*;
use std::fs::File;
use std::io::Read;
use std::io;
use std::f64;

use models::Entry;

impl Entry {
    pub fn convert_to_string(self) -> String {
        self.place + ";" +
        self.utc_date
            .format("%Y-%m-%d %H:%M:%S")
            .to_string()
            .as_str() + ";" + self.temperature.to_string().as_str()
    }
}

pub fn create_weather_entry(place: String, date: DateTime<Utc>, temperature: f64) -> Entry {
    Entry {
        place: place,
        utc_date: date,
        temperature: temperature,
    }
}

pub fn read_weather_entries(mut file: File) -> Vec<Entry> {
    let mut weather_entries: Vec<Entry> = Vec::new();
    let mut content: String = String::new();

    if let Err(e) = file.read_to_string(&mut content) {
        panic!("Error while reading: {:?}", e)
    }

    let content: Vec<&str> = content.split(|c| '\n' == c || '\r' == c).collect();

    for line in content {
        let split_line: Vec<&str> = line.split(';').collect();
        if split_line.len() == 3 {

            let place: &str = match split_line.get(0) {
                Some(s) => s,
                None => "",
            };

            let date = match split_line.get(1) {
                Some(s) => {
                    match Utc.datetime_from_str(s, "%Y-%m-%d %H:%M:%S") {
                        Ok(t) => t,
                        Err(_) => Utc::now(),
                    }
                }
                None => Utc::now(),
            };

            let temperature: f64 = match split_line.get(2) {
                Some(s) => {
                    match s.trim().parse() {
                        Ok(num) => num,
                        Err(_) => 0.0,
                    }
                }
                None => 0.0,
            };

            weather_entries.push(create_weather_entry(String::from(place), date, temperature));
        } else {
            println!("The following line is not in the expected format and will be ignored: {}",
                     line);
        }
    }

    weather_entries
}

pub fn input_weather_entry() -> Entry {
    let mut place = String::new();
    let date = Utc::now();
    let mut temp = String::new();

    println!("Where are you?");
    if let Err(e) = io::stdin().read_line(&mut place) {
        panic!("That doesn't work as expected: {}", e);
    }

    place = String::from(place.trim());

    println!("What is the temperature?");
    let temp: f64 = match io::stdin().read_line(&mut temp) {
        Ok(_) => {
            match temp.trim().parse() {
                Ok(e) => e,
                Err(_) => 0.0,
            }
        }
        Err(_) => 0.0,
    };

    create_weather_entry(place, date, temp)
}
