#[macro_use] extern crate rocket;

mod templates;
use templates::*;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .register("/", catchers![templates::not_found])
        .attach(Template::fairing())
}
