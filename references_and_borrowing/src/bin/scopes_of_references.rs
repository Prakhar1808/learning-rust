fn main() {
    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s; // mutiple immutable references aren't a problem
    println!("{r1} and {r2}");
    // r1 and r2 will not be used after this point.

    let r3 = &mut s;
    println!("{r3}");
}
