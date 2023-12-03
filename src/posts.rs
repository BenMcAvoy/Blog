use crate::render::render;

#[derive(Debug)]
pub struct Post {
    pub content: String,
    pub name: String,
    pub url: String,
}

impl Post {
    pub fn new(content: String, name: String, url: String) -> Post {
        Post {
            content,
            name,
            url,
        }
    }
}

pub fn load_posts(filenames: Vec<String>) -> Vec<Post> {
   filenames.into_iter().map(|filename| {
       let html = render(filename.clone());
       let path = format!("/posts/{}", filename);
       Post::new(html, String::from(""), path)
   }).collect()
}
