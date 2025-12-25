// Function definition
fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon = return value (or use return keyword)
}

// Function with multiple returns (tuple)
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

fn main() {
    let sum = add(5, 3);
    let (a, b) = swap(1, 2);
    
    // Functions are first-class citizens
    let func: fn(i32, i32) -> i32 = add;
    let result = func(10, 20);
}
