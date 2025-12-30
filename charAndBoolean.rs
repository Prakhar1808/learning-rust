fn main() {
    //Characters (4 bytes, Unicode)
    let heart = '❤️';
    let rocket = '🚀';
    let letter = 'z';

    println!("Emojis: {} {}", heart, rocket);

    //Character Operations
    let c1 = 'A';
    let c2 = 'B';

    //can't directly add chars like in C
    //but we can convert
    let c1_val = c1 as u32;
    let c2_val = c2 as u32;

    println!("'A' as u32 = {}",c1_val);
    println!("'B' as u32 = {}",c2_val);

    //Booleans
    let t = true;
    let f = false;

    //boolean operations
    println!("t && f = {}", t && f);
    println!("t || f = {}", t || f);
    println!("!t = {}", !t);

    //boolean from comparison
    let x = 5;
    let y = 10;
    let is_greater = x > y;
    println!("{} > {} = {}", x, y, is_greater);
}