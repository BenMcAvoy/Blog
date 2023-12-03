use std::fs;

use crate::render::render;
use serde::Serialize;

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
    // Split the input into lines and take the second line
    let lines: Vec<&str> = text.lines().collect();

    // TODO: Improve getting line
    let text = lines[2];

    // Convert the string into a vector of characters
    let chars: Vec<char> = text.chars().collect();

    // Convert the vector back into a string
    let result: String = chars.into_iter().collect();

    // Truncate the string to 250 characters
    let mut truncated_result = result;
    truncated_result.truncate(100);

    truncated_result + "..."
}

pub fn load_posts(filenames: Vec<String>) -> Vec<Post> {
    filenames
        .into_iter()
        .map(|filename| {
            let html = render(filename.clone(), None);
            let path = format!("/posts/{}", filename);

            let filepath = format!("./templates/posts/{filename}.md");
            dbg!(&filepath);
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
