#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn get_area(self: &Self) -> u32 {
        self.width * self.height
    }

    fn get_perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    fn can_hold(&self, rect2: &Rectangle) -> bool {
        self.width >= rect2.width && self.height >= rect2.height
    }
}

fn main() {
    let rect = Rectangle {
        width: 12,
        height: 10,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 10,
    };
    let rect3 = Rectangle {
        width: 13,
        height: 10,
    };

    println!("rect is {:?}", rect);
    println!("rect area is {}", rect.get_area());
    println!("rect perimeter is {}", rect.get_perimeter());

    println!("rect2 is {:?}", rect2);
    println!("rect1 can hold rect2? {}", rect.can_hold(&rect2));

    println!("rect3 is {:?}", rect3);
    println!("rect1 can hold rect3? {}", rect.can_hold(&rect3));
}
