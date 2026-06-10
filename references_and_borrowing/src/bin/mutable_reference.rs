fn main() {
    let mut s = String::from("hello");

    change(&mut s); // a mutable reference ladies and gentlemen!
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// you cannot make 2 mutable references to "s"
// you also cannot have a mutable reference with an immutable one and vice-versa
