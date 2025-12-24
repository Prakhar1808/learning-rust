fn main() {
    // If statements (no parentheses needed)
    let number = 6;
    
    if number % 2 == 0 {
        println!("even");
    } else {
        println!("odd");
    }
    
    // if is an expression (can return value, like Python ternary)
    let result = if number > 5 { "big" } else { "small" };
    
    // Loops
    // 1. loop (infinite until break)
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // break can return a value
        }
    };
    
    // 2. while
    while counter < 5 {
        println!("counter: {}", counter);
        counter += 1;
    }
    
    // 3. for (like Python's for, not C's)
    for i in 0..5 { // Range 0-4
        println!("{}", i);
    }
    
    // Pattern matching with match (like switch but more powerful)
    let value = 42;
    match value {
        0 => println!("zero"),
        1..=10 => println!("small"),
        11..=100 => println!("medium"),
        _ => println!("large"), // Default case
    }
}