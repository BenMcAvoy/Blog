use pulldown_cmark::{html, Options, Parser};

use rocket_dyn_templates::{
    context,
    tera::{Context, Tera},
    Template,
};

use rocket::*;

use crate::utils::calculate_age;

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

#[get("/post/<name>")]
pub fn get_post(name: String) -> Template {
    let path = format!("./templates/posts/{name}.md");
    let markdown = std::fs::read_to_string(path).expect("Valid post");

    let mut tera = Tera::default();
    tera.add_raw_template("post", &markdown).unwrap();

    let context = Context::new();
    let rendered_markdown = tera.render("post", &context).unwrap();

    let parser = Parser::new_ext(&rendered_markdown, Options::all());

    let mut html = String::new();
    html::push_html(&mut html, parser);

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
