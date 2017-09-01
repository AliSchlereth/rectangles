#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        if self.length > other_rect.length && self.width > other_rect.width {
            true
        } else {
            false
        }
    }
}


fn main() {
    let rect = Rectangle {length: 50, width: 30};
    // let rect2 = Rectangle {length: 25, ..rect};
    let rect2 = Rectangle {length: 40, width: 10};
    let rect3 = Rectangle {length: 45, width: 60};
    println!(
        "The area of the rectangle is {} square pixels.",
        rect.area()
    );
    println!("rect is {:#?}", rect);
    println!("Can rect hold rect2? {}", rect.can_hold(&rect2));
    println!("Can rect hold rect3? {}", rect.can_hold(&rect3));
}

// fn area (rectangle: &Rectangle) -> u32 {
//     rectangle.length * rectangle.width
// }
