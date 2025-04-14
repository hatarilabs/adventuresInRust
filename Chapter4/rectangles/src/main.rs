// method syntax
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // let width1 = 30;
    // let height1 = 50;
    let scale = 2;
    let rect1 = Rectangle {
        width: dbg!(30*scale),
        height: 50,
    };

    // println!("{}",area(&rect1));
    // println!("{:#?}",&rect1);
    dbg!(&rect1);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}