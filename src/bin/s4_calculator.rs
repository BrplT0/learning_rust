use std::io;
fn main() {
    let mut operation = String::new();
    println!("Welcome to Calculator!:\n");
    println!("Select the operation?\nAdd(A)\nSubtract(S)\nDivide(D)\nMultiply(M)");
    io::stdin().read_line(&mut operation).expect("Failed to read line");

    let mut a= String::new();
    let mut b= String::new();
    println!("Please enter the first number:");
    io::stdin().read_line(&mut a).expect("Failed to read line");
    println!("Please enter the second number:");
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let a: f64= a.trim().parse().unwrap_or(1.0);
    let b: f64= b.trim().parse().unwrap_or(1.0);

    match operation.trim().to_uppercase().as_str() {
        "A" => println!("Answer: {}", add(a, b)),
        "S" => println!("Answer: {}", subtract(a,b)),
        "D" => println!("Answer: {}", divide(a,b)),
        "M" => println!("Answer: {}", multiply(a,b)),
        _ => println!("Invalid operation"),
    }

}

fn add(a: f64, b: f64) -> f64 {
    a + b
}

fn subtract(a: f64, b: f64) -> f64 {
    a - b
}

fn divide(a: f64, b: f64) -> f64 {
    if b == 0.0 {
        println!("Error: Division by zero is not allowed.");
        0.0
    }
    else {
        a / b
    }
}

fn multiply(a: f64, b: f64) -> f64 {
    a * b
}