use chrono::prelude::DateTime;
use chrono::prelude::Local;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fs::File;
use std::io::Read;
use std::f64;

pub struct WeatherEntry {
    place: String,
    date: DateTime<Local>,
    temperature: f64,
}

impl Display for WeatherEntry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} , {} , {})", self.place, self.date, self.temperature)
    }
}

impl WeatherEntry {
    pub fn convert_to_string(self) -> String {                          //FIXME
        self.place + ";" + self.date.to_rfc3339().as_str() + ";" + stringify!(self.temperature)
    }
}

pub fn create_weather_entry(place: String, date: DateTime<Local>, temperature: f64) -> WeatherEntry {
    WeatherEntry {
        place: place,
        date: date,
        temperature: temperature,
    }
}

pub fn read_weather_entries(mut file: File) -> Vec<WeatherEntry> {
    let mut weather_entries: Vec<WeatherEntry> = Vec::new();
    let mut content: String = String::new();

    if let Err(e) = file.read_to_string(&mut content) {
        panic!("Error while reading: {:?}", e)
    }

    let content: Vec<&str> =  content.split(|c| '\n' == c || '\r' == c).collect();

    for line in content {
        let split_line: Vec<&str> = line.split(';').collect();
        if split_line.len() == 3 {

            let place: &str = match split_line.get(0) {
                Some(s) => s,
                None => "",
            };

            let date: DateTime<Local> = match split_line.get(1) {
                Some(s) => Local::now(), //TODO implementation
                None => Local::now(),
            };

            let temperature: f64 = match split_line.get(2) {
                Some(s) => match s.trim().parse() {
                    Ok(num) => num,
                    Err(_) => 0.0,
                },
                None => 0.0,
            };

            weather_entries.push(create_weather_entry(String::from(place),date,temperature));
        } else {
            println!("The following line is not in the expected format and will be ignored: {}", line);
        }
    }

    weather_entries
}
