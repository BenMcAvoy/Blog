use highlight_pulldown::highlight_with_theme;
use pulldown_cmark::{html, Parser};
use rocket_dyn_templates::tera::{Context, Tera};
use std::{borrow::Cow, fmt::Display};

const THEME: &str = "base16-eighties.dark";

/// Takes in the name of a page and renders it to HTML
/// as a string.
///
/// `S` is any type that is a string slice and implements
/// the Display trait.
pub fn render<S>(name: S, context: Option<Context>) -> String
where
    S: Into<Cow<'static, str>> + Display,
{
    let path = format!("./templates/posts/{name}.md");
    let markdown = std::fs::read_to_string(path).expect("Valid post");

    let mut tera = Tera::default();
    tera.add_raw_template("post", &markdown).unwrap();

    let context = context.unwrap_or_default();
    let rendered_markdown = tera.render("post", &context).unwrap();

    // let parser = Parser::new_ext(&rendered_markdown, Options::all());
    let events = Parser::new(&rendered_markdown);
    let events = highlight_with_theme(events, THEME).unwrap();

    let mut html = String::new();
    html::push_html(&mut html, events.into_iter());

    html
}
