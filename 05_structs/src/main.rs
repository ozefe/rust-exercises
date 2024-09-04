#![allow(dead_code)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct AlwaysEqual;

fn main() {
    let mut user1 = build_user(
        String::from("someone@example.com"),
        String::from("someusername123"),
    );
    println!("User1 email: {}", user1.email);

    user1.email = String::from("anotheremail@example.com");
    println!("User1 email: {}", user1.email);

    let user2 = User {
        email: String::from("another@example.com"),
        ..user1
    };
    println!("User2 email: {}", user2.email);

    // println!("User1 username: {}", user1.username);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
