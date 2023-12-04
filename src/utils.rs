use chrono::{Datelike, NaiveDate, Utc};

pub fn calculate_age(birthdate: Option<NaiveDate>) -> i32 {
    let birthdate = birthdate.expect("Date functions to work");
    let today = Utc::now().naive_utc();
    today.year() - birthdate.year()
}

fn capitalize(word: &&str) -> String {
    let word = word.to_owned().to_owned();

    match word.chars().next() {
        Some(first) => {
            let capitalized_first = first.to_uppercase();
            let rest = &word[1..].to_lowercase();
            format!("{}{}", capitalized_first, rest)
        }
        None => String::new(),
    }
}

pub fn snake_to_titlecase<T>(text: T) -> String
where
    T: ToString,
{
    let text = text.to_string();
    let words: Vec<&str> = text.split(' ').collect();
    let words: Vec<String> = words.iter().map(capitalize).collect();

    words.join(" ")
}
