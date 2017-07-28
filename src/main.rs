extern crate chrono;

mod weather;

use std::fs::File;
use std::io;
use std::io::Write;
use weather::WeatherEntry;

use chrono::prelude::Local;

fn main() {
    let mut input = 1;
    let mut target = match File::create("weather.data") {
        Ok(f) => f,
        Err(e) => panic!("{}", e),
    };

    while input != 9 {
        let w_entry: WeatherEntry;

        let mut place = String::new();
        let date = Local::now();
        let mut temp = String::new();

        println!("Where are you?");
        if let Err(e) = io::stdin().read_line(&mut place) {
            panic!("That doesn't work as expected: {}",e);
        }

        println!("What is the temperature?");
        let temp: f64 = match io::stdin().read_line(&mut temp) {
            Ok(_) => match temp.trim().parse() {
                Ok(e) => e,
                Err(_) => 0.0,
            },
            Err(_) => 0.0,
        };

        w_entry = weather::create_weather_entry(place, date, temp);

        if let Err(e) = target.write(w_entry.convert_to_string().as_bytes()){
            println!("Err while writing: {}", e);
        };

        input = command();
    }
}

//TODO IMPLEMENT
fn command() -> u64 {
    9
}
