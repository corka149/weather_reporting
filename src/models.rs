use std::fmt::Result;
use std::fmt::Formatter;
use std::fmt::Display;
use chrono::NaiveDateTime;

#[derive(Queryable)]
pub struct Entry {
    pub place: String,
    pub utc_date: NaiveDateTime,
    pub temperature: f64,
}

impl Display for Entry {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({} , {} , {})", self.place, self.utc_date, self.temperature)
    }
}