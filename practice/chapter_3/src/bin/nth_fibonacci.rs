// Generate the nth Fibonacci number
use std::io::{self, Write};

fn main() {
    print!("Input n: ");
    io::stdout().flush().unwrap(); // just to flush out the bufferedline

    let mut inputn = String::new();
    io::stdin()
        .read_line(&mut inputn)
        .expect("Failed to read line");

    let inputn: u32 = inputn
        .trim()
        .parse()
        .expect("not a number dawg!");

    let mut first = 0u64;
    let mut second = 1u64;

    if inputn == 0 {
        println!("The {inputn}th fibonacci number is: 0");
        return;
    }

    if inputn == 1 {
        println!("The {inputn}th fibonacci number is: 1");
        return;
    }

    for _ in 2..=inputn {
        let next:u64 = first + second;
        first = second;
        second = next;
    }

    print!("The {inputn}th fibonacci number is: {second}");
}
