fn main() {
    // Nested if-else
    let number = 15;
    
    if number % 2 == 0 {
        println!("{} is even", number);
        if number % 4 == 0 {
            println!("{} is divisible by 4", number);
        } else {
            println!("{} is not divisible by 4", number);
        }
    } else {
        println!("{} is odd", number);
        if number % 3 == 0 {
            println!("{} is divisible by 3", number);
        } else {
            println!("{} is not divisible by 3", number);
        }
    }
    
    // Chained else-if (like Python)
    let score = 85;
    
    let grade = if score >= 90 {
        'A'
    } else if score >= 80 {
        'B'
    } else if score >= 70 {
        'C'
    } else if score >= 60 {
        'D'
    } else {
        'F'
    };
    
    println!("Score: {}, Grade: {}", score, grade);
    
    // Ternary-style with if-else (common pattern)
    let age = 20;
    let status = if age >= 18 { "adult" } else { "minor" };
    println!("Age {} is {}", age, status);
}