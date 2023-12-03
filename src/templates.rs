use rocket_dyn_templates::{context, Template};
use rocket::*;

use crate::{render::render, utils::calculate_age};

const BIRTHDATE: Option<chrono::NaiveDate> = chrono::NaiveDate::from_ymd_opt(2009, 3, 13);

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            age: calculate_age(BIRTHDATE),
        },
    )
}

#[get("/posts")]
pub fn get_posts() -> Template {
    let html = render("index");

    Template::render(
        "post",
        context! {
            html
        },
    )
}

#[get("/posts/<name>")]
pub fn get_post(name: String) -> Template {
    let html = render(name);

    Template::render(
        "post",
        context! {
            html
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "error/404",
        context! {
            uri: req.uri()
        },
    )
}
