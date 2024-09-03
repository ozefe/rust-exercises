fn main() {
    let mut s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{s1}' is {len}.");

    change(&mut s1);
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
