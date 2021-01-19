#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle { width: size, height: size}
    }
}
fn main() {
    let rect1 = Rectangle::square(3);
    let rect2 = Rectangle::square(2);
    let rect3 = Rectangle::square(4);

    println!("rect1 in rect2 ? {} ", rect1.can_hold(&rect2));
    println!("rect1 in rect3 ? {} ", rect1.can_hold(&rect3));

    println! (
        "Rectangles Area: {} square pixel",
        rect1.area()
    );
}

