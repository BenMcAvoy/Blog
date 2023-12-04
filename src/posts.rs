use std::fs;

use crate::render::render;
use serde::Serialize;

const DEFAULT_SUMMARY: &str = "Sorry, we couldn't generate a summary for this";

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

fn generate_summary(text: String) -> String {
    let result: String = text
        .lines()
        .nth(2)
        .unwrap_or(DEFAULT_SUMMARY)
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

            let name: String = filename
                .split('-')
                .map(|word| {
                    format!(
                        "{}{}",
                        word.chars().next().unwrap().to_uppercase(),
                        &word[1..]
                    )
                })
                .collect::<Vec<String>>()
                .join(" ");

            Post::new(html, summary, name, path)
        })
        .collect()
}
