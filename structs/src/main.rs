struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("PubgLover1221"),
        email: String::from("janedoe@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("whathisname@example.com");
    let email = user1.email;
    println!("user email: {email}");
}
