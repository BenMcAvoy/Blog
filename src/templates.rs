use rocket_dyn_templates::{Template, context};
use rocket::*;

use chrono::Datelike;
use chrono::{NaiveDate, Utc};

use crate::utils::calculate_age;

const BIRTHDATE: Option<NaiveDate> = NaiveDate::from_ymd_opt(2009, 3, 13);

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {
        age: calculate_age(BIRTHDATE),
    })
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}
