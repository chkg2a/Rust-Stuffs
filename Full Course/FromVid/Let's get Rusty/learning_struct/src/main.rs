struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn main() {
    let user1 = User{
        username: String::from("BRUh"),
        email: String::from("herllo@gmail.com"),
        sign_in_count: 12,
        active: true
    };
    println!("{}",user1);
}
