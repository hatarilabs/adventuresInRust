// antes de multiplo imple¿ blocks
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// impl Rectangle {
//     fn area(&self) -> u32 {
//         self.width * self.height
//     }
// }

impl Rectangle {
    fn width(&self) -> bool {
        self.width > 0
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    // println!("{}",rect1.area());
    // if rect1.width() {
    //     println!("{}",rect1.width)
    // }
    println!("{}",rect1.can_hold(&rect2));
    println!("{}",rect1.can_hold(&rect3)); 
}
