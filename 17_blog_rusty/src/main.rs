use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");

    let mut post = post.request_review();

    post.approve();
    let post = post.approve().unwrap();

    assert_eq!("I ate a salad for lunch today", post.content());
}
