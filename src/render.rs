use highlight_pulldown::highlight_with_theme;
use pulldown_cmark::{html, Parser, Options};
use rocket_dyn_templates::tera::{Context, Tera};
use std::{borrow::Cow, fmt::Display, fs::read_to_string};

/// The theme to use for rendering code snippets.
const THEME: &str = "base16-eighties.dark";

/// Takes in the name of a page and renders it to HTML
/// as a string.
///
/// `S` is any type that can be made into a string slice
/// and implements the `Display` trait.
pub fn render<S>(name: S, context: Option<Context>) -> String
where
    S: Into<Cow<'static, str>> + Display,
{
    let path = format!("./posts/{name}.md");
    let markdown = read_to_string(&path).unwrap_or_else(|_| panic!("Invalid post {path}"));

    let mut tera = Tera::default();
    tera.add_raw_template("post", &markdown).unwrap();

    let context = context.unwrap_or_default();
    let rendered_markdown = tera.render("post", &context).unwrap();

    let parser = Parser::new_ext(&rendered_markdown, Options::all());
    let events = highlight_with_theme(parser, THEME).unwrap();

    let mut html = String::new();
    html::push_html(&mut html, events.into_iter());

    html
}
