#[macro_use] extern crate rocket;

use rocket::fs::FileServer;

mod templates;
use templates::*;

use rocket_dyn_templates::Template;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![templates::not_found])
        .mount("/public", FileServer::from("public/"))
        .mount("/", routes![index])
        .attach(Template::fairing())
}
