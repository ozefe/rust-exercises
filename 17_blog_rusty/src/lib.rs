pub struct Post {
    content: String,
}

impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

pub struct DraftPost {
    content: String,
}

impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn request_review(self) -> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
            approve_count: 0,
        }
    }
}

pub struct PendingReviewPost {
    content: String,
    approve_count: u8,
}

impl PendingReviewPost {
    pub fn approve(&mut self) -> Option<Post> {
        if self.approve_count >= 1 {
            Some(Post {
                content: self.content.to_owned(),
            })
        } else {
            self.approve_count += 1;
            None
        }
    }

    pub fn reject(self) -> DraftPost {
        DraftPost {
            content: self.content,
        }
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
    fn post_add_text() {
        let mut post = Post::new();
        post.add_text("test");
    }

    #[test]
    fn post_request_review() {
        let post = Post::new();
        post.request_review();
    }

    #[test]
    fn post_single_approve_does_not_approve() {
        let mut post = Post::new();
        post.add_text("test");

        let mut post = post.request_review();

        assert_eq!(post.approve().is_none(), true);
    }

    #[test]
    fn post_double_approve_does_approve() {
        let mut post = Post::new();
        post.add_text("test");

        let mut post = post.request_review();
        post.approve();

        assert_eq!(post.approve().is_some(), true);
    }

    #[test]
    fn post_reject() {
        let mut post = Post::new();
        post.add_text("test");

        let mut post = post.request_review();
        post.approve();
        let post = post.reject();

        let mut post = post.request_review();
        post.approve();

        assert_eq!(post.approve().is_some(), true);
    }
}
