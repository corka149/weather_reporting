use std::fmt::Result;
use chrono::prelude::*;
use std::fmt::Formatter;
use std::fmt::Display;

#[derive(Queryable)]
pub struct Entry {
    pub place: String,
    pub utc_date: DateTime<Utc>,
    pub temperature: f64,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} , {} , {})", self.place, self.utc_date, self.temperature)
    }
}