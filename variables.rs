fn main() {
    //immutable by default
    let x=5;
    // x=6; // ERROR: cannot assign twice to immutable variable
    

    //mutable variables need explicit 'mut'
    let mut y = 10;
    y=15; //no error


    //Type annotations (optional, like c but with inference)
    let z: i32 = 20;

    //Shadowing - redefining variable in same scope
    let a=5;
    let a=a+1; // New Variable, can change type
    let a="I have evolved twin, this is my final form, i am a string"; // works with shadowing

}
