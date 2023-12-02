#[macro_use] extern crate rocket;

use rocket::fs::{FileServer, NamedFile};

mod templates;
use templates::*;

use rocket_dyn_templates::Template;

#[get("/robots.txt")]
async fn robots() -> Option<NamedFile> {
    NamedFile::open("public/robots.txt").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![templates::not_found])
        .mount("/public", FileServer::from("public/"))
        .mount("/", routes![index, robots])
        .attach(Template::fairing())
}
