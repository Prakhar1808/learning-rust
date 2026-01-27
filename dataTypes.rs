fn main(){
    //scalar types (Like C Primitives)
    let int: i32 = -5           //Signed 32-bit (also i8, i16, i64, i128)
    let uint u32 = 5;           //Unsigned
    let float: f64 = 3.14;      //Also f32
    let boolean: bool = true;
    let character: char = 'z';  //unicode, 4 bytes (unlike C's char)

    // Compound types
    let tuple: (i32, f64, char) = (500, 6.4, 'J');
    let (x,y,z) = tuple:        //destructing (like Python)

    let array: [i32; 5] = [1, 2, 3, 4, 5]; //fixed size stack allocated
    let slice: &[i32] = &array[1..3]        //Reference to portion

    //String Types
    let string_literal: &str = "hello"; //immutable reference, stack
    let string_object: String = String::from("hello"); //Heap Allocated, growable
}

fn integerTypes() {
    //use u for unsigned and i for signed integers,
    //the number next to it tells about the length in bits
    //Eg: 8-bit usigned int a:
    let a: u8 = 25;
}

fn addingDifferentNumberSystems(){
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111; //1024 + 255 + 63 + 255
    //1_024 (the underscore is a delimiter)
    //decimal + hexadecimal + octal + binary
    assert_eq!(v == 1597);

    println!("Success!");
}
