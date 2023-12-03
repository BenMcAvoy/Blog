use rocket::*;
use rocket_dyn_templates::{context, tera::Context, Template};

use crate::{render::render, utils::calculate_age, PostStorage};

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
pub fn get_posts(state: &State<PostStorage>) -> Template {
    let mut context = Context::default();
    context.insert("posts", &state.posts);
    let html = render("index", Some(context));

    Template::render("post", context! { html })
}

#[get("/posts/<name>")]
pub fn get_post(name: String) -> Template {
    let html = render(name, None);

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

#[catch(500)]
pub fn internal_error(_: &Request<'_>) -> Template {
    Template::render("error/500", context! {})
}
