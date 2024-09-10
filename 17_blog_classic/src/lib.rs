use std::borrow::BorrowMut;

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        if self.state.as_ref().unwrap().can_edit() {
            self.content.push_str(text);
        }
    }

    pub fn content(&self) -> &str {
        self.state.as_ref().unwrap().content(self)
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review());
        }
    }

    pub fn approve(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }

    pub fn reject(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

impl Default for Post {
    fn default() -> Self {
        Self::new()
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, _post: &'a Post) -> &'a str {
        ""
    }
    fn can_edit(&self) -> bool {
        false
    }
}

struct Draft {}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { approve_count: 0 })
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn can_edit(&self) -> bool {
        true
    }
}

struct PendingReview {
    approve_count: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(mut self: Box<Self>) -> Box<dyn State> {
        if self.approve_count >= 1 {
            Box::new(Published {})
        } else {
            *self.approve_count.borrow_mut() += 1;
            self
        }
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}

struct Published {}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }

    fn content<'a>(&self, post: &'a Post) -> &'a str {
        &post.content
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn post_create() {
        Post::new();
    }

    #[test]
    fn post_empty_content_on_creation() {
        let post = Post::new();
        assert_eq!(post.content(), "");
    }

    #[test]
    fn post_add_text() {
        let mut post = Post::new();
        post.add_text("test");
    }

    #[test]
    fn post_request_review() {
        let mut post = Post::new();
        post.request_review();
    }

    #[test]
    fn post_empty_content_on_request_review() {
        let mut post = Post::new();
        post.request_review();

        assert_eq!(post.content(), "");
    }

    #[test]
    fn post_cannot_add_text_on_request_review() {
        let mut post = Post::new();
        post.request_review();

        post.add_text("test");

        assert_eq!(post.content(), "");
    }

    #[test]
    fn post_single_approve_does_not_approve() {
        let mut post = Post::new();
        post.add_text("test");

        post.request_review();
        post.approve();

        assert_eq!(post.content(), "");
    }

    #[test]
    fn post_double_approve_does_approve() {
        let mut post = Post::new();
        post.add_text("test");

        post.request_review();
        post.approve();
        post.approve();

        assert_eq!(post.content(), "test");
    }

    #[test]
    fn post_reject() {
        let mut post = Post::new();
        post.add_text("test");

        post.request_review();
        post.approve();
        post.reject();

        assert_eq!(post.content(), "");

        post.request_review();
        post.approve();
        post.approve();

        assert_eq!(post.content(), "test");
    }
}
