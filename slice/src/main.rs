fn main() {
    println!("String Slice");
}

// fn first_word(s: &String) -> &str { // old line
fn first_word(s: &str) -> &str { // new one allows us to use the fun on `&String` and `&str` both
                                 // now compatible with both string variables and string literals
    let bytes = s.as_bytes();

    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
