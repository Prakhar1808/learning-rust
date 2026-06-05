// Convert Temperatures between Fahrenheit and Celsius
use std::io;

fn f_to_c(temperature: f64) -> f64 {
    (temperature - 32.0) / 1.8
}

fn c_to_f(temperature: f64) -> f64 {
    (temperature * 1.8) + 32.0
}

fn main() {
    println!("Choose 1 to convert Fahrenheit to Celsius and ");
    println!("Choose 2 to convert Celsius to Fahrenheit:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let num: i32 = input
        .trim()
        .parse()
        .expect("Please type a valid number!");

    println!("Your input was: {num}");

    if num ==1 {
        let fahrenheit = 98.4;
        let fresult = f_to_c(fahrenheit);
        println!("Converted Value in Fahrenheit: {fresult}");
    }
    else if num == 2 {
        let celsius = 24.0;
        let cresult = c_to_f(celsius);
        println!("Converted Value in Celsius: {cresult}");
    }
    else {
        println!("Incorrect input!");
    }
}
