#![allow(dead_code)]

fn main() {
    // let v: Vec<i32> = Vec::new();
    // let v = vec![1, 2, 3];
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("Vector: {v:?}");

    let v = vec![1, 2, 3, 4, 5];
    println!("Vector: {v:?}");

    let third: &i32 = &v[2];
    println!("The third element is {third}");

    let third: Option<&i32> = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    // This panics since `v` only has 5 items
    // let _does_not_exist = &v[100];
    // This does not panic because it returns an `Option`, indicating it could be `None`
    let _does_not_exist = v.get(100);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    let mut v = v;
    for i in &mut v {
        *i += 50;
    }
    println!("Vector: {v:?}");

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("Row: {row:?}")
}
