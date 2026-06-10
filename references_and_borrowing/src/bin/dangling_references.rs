fn main() {
    let reference_to_nothing = dangle();
    println!("Code has error to demonstrate dangling references");
}

fn dangle() -> &String { // referencing to a string out of scope
                         // *facepalms*
    let s = String::from("hello");

    &s // rather return the String directly
}
