fn main() {
    const DAYS: [&str; 12] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    const NUMBERS: [&str; 12] = [
        "A", "Two", "Three", "Four", "Five", "Six", "Seven", "Eight", "Nine", "Ten", "Eleven",
        "Twelve",
    ];
    const LYRICS: [&str; 12] = [
        "partridge in a pear tree",
        "turtle doves",
        "French hens",
        "calling birds",
        "golden rings",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for (i, day) in DAYS.iter().enumerate() {
        println!("On the {day} day of Christmas,");
        println!("my true love gave to me");

        for j in (1..=i).rev() {
            println!("{} {},", NUMBERS.get(j).unwrap(), LYRICS.get(j).unwrap());
        }
        println!(
            "{} {}.",
            if i == 0 {
                NUMBERS.get(0).unwrap()
            } else {
                "And a"
            },
            LYRICS.get(0).unwrap()
        );

        println!();
    }
}
