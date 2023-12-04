use rocket::*;
use rocket_dyn_templates::{context, Template};

use crate::{render::render, utils::calculate_age, PostStorage};

const BIRTHDATE: Option<chrono::NaiveDate> = chrono::NaiveDate::from_ymd_opt(2009, 3, 13);

#[get("/")]
pub fn index() -> Template {
    Template::render(
        "index",
        context! {
            age: calculate_age(BIRTHDATE),
            page: "home",
        },
    )
}

#[get("/posts")]
pub fn get_posts(state: &State<PostStorage>) -> Template {
    let posts = &state.posts;

    Template::render("posts", context! { posts, page: "posts" })
}

#[get("/posts/<name>")]
pub fn get_post(name: String) -> Template {
    let html = render(name, None);

    Template::render(
        "post",
        context! {
            page: "posts",
            html,
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

#[catch(500)]
pub fn internal_error(_: &Request<'_>) -> Template {
    Template::render("error/500", context! {})
}
