#[macro_use]
extern crate rocket;

use posts::{load_posts, Post};
use rocket::fs::{FileServer, NamedFile};

mod templates;
use templates::*;

mod posts;
mod render;
mod utils;

use rocket_dyn_templates::Template;

pub struct PostStorage {
    pub posts: Vec<Post>,
}

#[get("/robots.txt")]
async fn robots() -> Option<NamedFile> {
    NamedFile::open("public/robots.txt").await.ok()
}

#[launch]
fn rocket() -> _ {
    let paths = std::fs::read_dir("./posts")
        .expect("Templates directory to exist")
        .filter_map(Result::ok)
        .filter(|e| e.path().extension() == Some(std::ffi::OsStr::new("md")))
        .filter(|e| e.path().file_stem().unwrap().to_str().unwrap() != "index")
        .map(|e| e.path().file_stem().unwrap().to_str().unwrap().to_string())
        .collect::<Vec<_>>();

    let posts = load_posts(paths);

    rocket::build()
        .manage(PostStorage { posts })
        .register("/", catchers![not_found, internal_error])
        .mount("/public", FileServer::from("public/"))
        .mount("/", routes![index, robots, get_post, get_posts])
        .attach(Template::fairing())
}
