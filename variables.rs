fn main() {
    //immutable by default
    let x=5;
    // x=6; // ERROR: cannot assign twice to immutable variable
    assert_eq!(x, 5);
    println!("Success!");


    //mutable variables need explicit 'mut'
    let mut y = 10;
    y+=15; //no error //25
    assert_eq!(y, 25);
    println!("Success!");

    //Type annotations (optional, like c but with inference)
    let z: i32 = 20;

    //Shadowing - redefining variable in same scope
    let a=5;
    let a=a+1; // New Variable, can change type
    let a="I have evolved twin, this is my final form, i am a string"; // works with shadowing
    //more about shadowing
    let i: i32 = 5;
    {
        let i= 12;
        assert_eq!(i, 12);
    }

    assert_eq!(i, 5);
    let i = 40;
    assert_eq!(i, 40);
    println!("{}", x); //40

    let w: i32; //this will give a warning, because uninitialized but un used
}
