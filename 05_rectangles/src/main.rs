#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.method_area() >= other.method_area()
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

impl Rectangle {
    fn method_area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    const WIDTH: u32 = 30;
    const HEIGHT: u32 = 50;

    let width1 = WIDTH;
    let height1 = HEIGHT;
    println!(
        "NORMAL AREA: The area of the rectangle is {} square pixels.",
        normal_area(width1, height1)
    );

    let rect1 = (WIDTH, HEIGHT);
    println!(
        "TUPLE AREA: The area of the rectangle is {} square pixels.",
        tuple_area(rect1)
    );

    let rect2 = Rectangle {
        width: WIDTH,
        height: HEIGHT,
    };
    println!(
        "STRUCT AREA: The area of the rectangle is {} square pixels.",
        struct_area(&rect2)
    );

    println!("rect2 is {rect2:#?}");

    let scale = 2;
    let rect3 = Rectangle {
        width: dbg!(WIDTH * scale),
        height: HEIGHT,
    };
    dbg!(&rect3);

    println!(
        "METHOD AREA: The area of the rectangle is {} square pixels.",
        rect2.method_area()
    );

    if rect2.width() {
        println!("The rect2 has a positive width; it is {}", rect2.width);
    }

    let rect1 = Rectangle {
        width: WIDTH,
        height: HEIGHT,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let sq = Rectangle::square(3);
    dbg!(sq);
}

fn normal_area(width: u32, height: u32) -> u32 {
    width * height
}

fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
