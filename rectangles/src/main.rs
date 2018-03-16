fn main() {
    let main_rectangle = Rectangle { height: 30, width: 50 };
    let rect2 = Rectangle { height: 25, width: 32 };
    let rect3 = Rectangle { height: 52, width: 40 };

    println!("Can main_rectangle hold rect2? {}", main_rectangle.can_hold(&rect2));
    println!("Can main_rectangle hold rect3? {}", main_rectangle.can_hold(&rect3));
}



#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32
}

impl Rectangle {
    fn build_square(length: u32) -> Rectangle {
       Rectangle { height: length, width: length } 
    }

    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn display_area(&self) {
        println!("My area is {} square pixels.", self.area());
    }

    fn can_hold(&self, rect: &Rectangle) -> &str {
        if self.height > rect.height && self.width > rect.width {
            "Yes"
        } else {
            "No"
        }
    }
}
