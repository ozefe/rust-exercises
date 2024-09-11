fn main() {
    // Doesn't compile because `let` statement takes an irrefutable pattern
    // let Some(_x): Option<u8> = None;

    let value: Option<u8> = None;
    if let Some(x) = value {
        println!("{x}");
    }

    // Compiler warns us that `if let` expression is unnecessary here since
    // we're using an irrefutable pattern.
    // if let x = 5 {
    //     println!("{x}");
    // }
}
