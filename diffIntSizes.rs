fn main() {
    // Different Interger Sizes
    let small: i8 = 127; //Max Value for i8
    let medium: i16 = 3200;
    let normal: i32 = 2000000000; // 10 digit integer
    let big: i64 = 9000000000000000000; //19 digit integer
    

    // there's also overflow (Checked in debug mode)
    // let overflow = small + 1; // this would panic in debug mode


    // Bitwise Operations
    let x: u8 = 0b1010_1010; // Binary literal
    let y: u8 = 0b1100_1100;


     println!("x & y = {:08b}", x & y); // Bitwise AND
    println!("x | y = {:08b}", x | y); // Bitwise OR
    println!("x ^ y = {:08b}", x ^ y); // Bitwise XOR
    println!("!x = {:08b}", !x);      // Bitwise NOT
    println!("x << 2 = {:08b}", x << 2); // Left shift
    println!("y >> 2 = {:08b}", y >> 2); // Right shift


    //TBC
