struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        // field init shorthand
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
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

    // normal syntax
    // let user2 = User {
    //    active: user1.active, // new `User` instance using a value from user1
    //    username: user1.username,
    //    email: String::from("johnqpublic@example.com"),
    //    sign_in_count: user1.sign_in_count,
    // };

    // struct update syntax
    let user2 = User {
        email: String::from("johnqpublic@example.com"),
        ..user1 // user1 cannot be used now, because of username
                // if we had only copied `active` and `sign_in_count` we could still use user1
                // because of the copy trait
                // user1.email can still be used because it has not moved out
    };
}
