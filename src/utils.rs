use chrono::{NaiveDate, Utc, Datelike};

pub fn calculate_age(birthdate: Option<NaiveDate>) -> i32 {
    let birthdate = birthdate.expect("Date functions to work");
    let today = Utc::now().naive_utc();
    today.year() - birthdate.year()
}
