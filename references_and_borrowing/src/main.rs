fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);
    // &s1 syntax lets us create a reference that refers to the value of s1
    // but does not own it
    // because its a reference and it doesn't own it, the value won't be dropped
    // when reference stops being used
    // this is called borrowing

    println!("The length of '{s1}' is {len}.");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

// borrowed values cannot be modified directly!
// check /bin for more *wink*
