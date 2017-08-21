extern crate diesel;
extern crate weather_reporting;

use self::weather_reporting::*;
use self::weather_reporting::models::*;
use self::diesel::prelude::*;

fn main(){
    use weather_reporting::schema::entries::dsl::*;

    let connection = establish_connection();
    let results = entries.limit(5)
        .load::<Entry>(&connection)
        .expect("Error loading entries");

    println!("Displaying {} weather entries", results.len());

    for entry in results {
        println!("{}", entry);
    }
}