struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("Hello, world!");

    let a = build_user(String::from("another2@example.com"),String::from("anotheruser3"));
    println!("{}",a.username);

    let user2 = User {
        email : String::from("another@example.com"),
        ..user1
    };
    println!("{}",user2.email);
   
}

fn build_user(email: String, username:String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count:1,
    }
}