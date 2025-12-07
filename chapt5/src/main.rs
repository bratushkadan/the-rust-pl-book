#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated functions
    // Associated functions are often used as the constructors
    fn square(side: u32) -> Rectangle {
        Rectangle { width: side, height: side }
    }
    // Methods immutable borrowing
    fn area(&self) -> u32 {
        self.width * self.height    
    }
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width >= rect.width && self.height >= rect.height
    }
    // Methods can take ownership:
    fn take_ownership(self) {
    }
    // Methods can borrow mutable:
    fn borrow_mut(&mut self) {
    }
}

fn main() {
    let rect = Rectangle {
        width: 15,
        height: 7,
    };

    println!("area of a rect is {}", rect.area());
    println!("perimeter of a rect is {}", rect.perimeter());

    println!("{:?}", &rect);
    println!("{:#?}", &rect);
    println!("{:#?}", Rectangle::square(4));
}