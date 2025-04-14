struct Color(i32,i32,i32);
struct Point(i32,i32,i32);
    
fn main() {
    let black = Color(0, 10, 0);
    let origin = Point(20, 0, 0);
    println!("{}",black.1);
    println!("{}",origin.0);
}
