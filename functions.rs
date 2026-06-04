// Function definition
fn add(x: i32, y: i32) -> i32 {
    x + y // No semicolon = return value (or use return keyword)
}

// Function with multiple returns (tuple)
fn swap(x: i32, y: i32) -> (i32, i32) {
    (y, x)
}

// parameters
fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

// statements and expressions
// rust is an expression-based language
fn five() -> i32 {
    5
}

fn main() {
    let sum = add(5, 3);
    let (a, b) = swap(1, 2);

    // Functions are first-class citizens
    let func: fn(i32, i32) -> i32 = add;
    let result = func(10, 20);

    // parameters
    print_labeled_measurement(5, 'h');

    // statements and expressions
    let y = {
            let x = 3;
            x + 1 // no semicolon
                // expressions don't include ending semicolons
                // adding a semicolon will make it a statement
                // but statements don't return values in rust unlike in C or Ruby
                // x = y = 6 doesn't make sense in rust but does in C and ..
                // evaluates to 4 btw, bound to y
    }

    // Functions with Return Values
    let f = five();
    println!("The value of x is: {x}");
}
