use rocket_dyn_templates::{Template, context};
use rocket::*;

#[get("/")]
pub fn index() -> Template {
    Template::render("index", context! {})
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render("error/404", context! {
        uri: req.uri()
    })
}
