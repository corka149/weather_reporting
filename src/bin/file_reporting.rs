extern crate chrono;
extern crate weather_reporting;

use std::fs::File;
use std::io;
use std::io::Write;
use weather_reporting::models::Entry;
use weather_reporting::operations::file;
use std::fs::OpenOptions;
use std::ops::Add;

fn main() {
    let mut input = command();
    let mut target: File;

    while input != 9 {
        match input {
            1 => {
                target = match OpenOptions::new().append(true).open("weather.data") {
                    Ok(f) => f,
                    Err(e) => panic!("{}", e),
                };
                let w_entry: Entry = file::input_weather_entry();
                let weather_string: String = String::from("\n");
                let weather_string: String = weather_string.add(w_entry.convert_to_string().as_str());

                if let Err(e) = target.write(weather_string.as_bytes()) {
                    println!("Err while writing: {}", e);
                };
            }
            2 => {
                target = match File::open("weather.data") {
                    Ok(f) => f,
                    Err(e) => panic!("{}", e),
                };
                let entries = file::read_weather_entries(target);
                for e in entries {
                    println!("{}", e);
                }
            }
            _ => {}
        }
        input = command();
    }
}

fn command() -> u64 {
    let mut input: String = String::new();

    println!("Select a command");
    println!("1: Input new entry");
    println!("2: Read weather entries");
    println!("9: Exit program");

    let input: u64 = match io::stdin().read_line(&mut input) {
        Ok(_) => {
            match input.trim().parse() {
                Ok(e) => e,
                Err(p_e) => {
                    println!("{}", p_e);
                    9
                }
            }
        }
        Err(r_e) => {
            println!("{}", r_e);
            9
        }
    };

    if input != 1 && input != 2 && input != 9 {
        command();
    }
    input
}
