use crate::model;

use model::Post;

#[derive(Clone, Debug)]
pub struct Database {
    posts: Vec<Post>,
}

impl Database {

    pub fn new() -> Database {
        return Database { posts: vec![] };
    }

    pub fn insert_post(&mut self, post: Post) {
        self.posts.push(post);
    }

    pub fn posts(&self) -> &Vec<Post> {
        return &self.posts
    }
}


