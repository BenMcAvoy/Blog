#[macro_use]
extern crate rocket;

use rocket::fs::{FileServer, NamedFile};

mod templates;
use templates::*;

mod render;
mod utils;

use rocket_dyn_templates::Template;

#[get("/robots.txt")]
async fn robots() -> Option<NamedFile> {
    NamedFile::open("public/robots.txt").await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .register("/", catchers![not_found, internal_error])
        .mount("/public", FileServer::from("public/"))
        .mount("/", routes![index, robots, get_post, get_posts])
        .attach(Template::fairing())
}
