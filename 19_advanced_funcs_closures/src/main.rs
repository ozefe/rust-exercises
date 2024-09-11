#![allow(dead_code)]

fn add_one(x: i32) -> i32 {
    x + 1
}

fn add_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

enum Status {
    Value(u32),
    Stop,
}

fn main() {
    let answer = add_twice(add_one, 5);

    println!("The answer is: {answer}");

    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();
}

fn _returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
