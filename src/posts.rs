use std::fs;

use crate::{render::render, utils::snake_to_titlecase};
use serde::Serialize;
use std::error::Error;
use regex::Regex;

const DEFAULT_SUMMARY: &str = "Sorry, we couldn't generate a summary for this";
const DESCRIPTION_PATTERN: &str = "<div class=\"title\">\\s*#\\s*.*\n([^<]*)\\s*<\\/div>";

#[derive(Debug, Serialize)]
pub struct Post {
    pub content: String,
    pub summary: String,
    pub name: String,
    pub url: String,
}

impl Post {
    pub fn new(content: String, summary: String, name: String, url: String) -> Post {
        Post {
            content,
            summary,
            name,
            url,
        }
    }
}


fn get_line_under_header(html: &str) -> Result<String, Box<dyn Error>> {
    let regex = Regex::new(DESCRIPTION_PATTERN)?;
    let groups = regex.captures(html).ok_or("No match found")?;

    if let Some(matched_text) = groups.get(1) {
        return Ok(matched_text.as_str().to_string())
    }

    Err("No match found".into())
}

fn generate_summary(text: String) -> String {
    let result: String = get_line_under_header(&text)
        .unwrap_or(DEFAULT_SUMMARY.into())
        .chars()
        .take(100)
        .collect();

    result.trim_end().to_owned() + "..."
}

pub fn load_posts(filenames: Vec<String>) -> Vec<Post> {
    filenames
        .into_iter()
        .map(|filename| {
            let html = render(filename.clone(), None);
            let path = format!("/posts/{}", filename);

            let filepath = format!("./posts/{filename}.md");
            let text = fs::read_to_string(filepath).expect("Valid path");
            let summary = generate_summary(text);

            let name = snake_to_titlecase(filename);

            Post::new(html, summary, name, path)
        })
    .collect()
}
