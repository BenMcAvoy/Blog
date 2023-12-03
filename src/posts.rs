use crate::render::render;

#[derive(Debug)]
pub struct Post {
    pub content: String,
    pub name: String,
    pub url: String,
}

impl Post {
    pub fn new(content: String, name: String, url: String) -> Post {
        Post { content, name, url }
    }
}

pub fn load_posts(filenames: Vec<String>) -> Vec<Post> {
    filenames
        .into_iter()
        .map(|filename| {
            let html = render(filename.clone());
            let path = format!("/posts/{}", filename);

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

            Post::new(html, name, path)
        })
        .collect()
}
