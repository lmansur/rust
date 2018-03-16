fn main() {
    let user = User {
        username: String::from("lmansur"),
        email: String::from("lucas.mansur2@gmail.com"),
        active: true,
        sign_in_count: 1,
    };

    println!("=============================");
    println!("username: {}", user.username);
    println!("email: {}", user.email);
    println!("active: {}", user.active);
    println!("sign_in_count: {}", user.sign_in_count);
    println!("=============================");
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
