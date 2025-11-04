use std::io;
fn main() {
    let mut n = String::new();
    println!("Enter the n:");
    io::stdin().read_line(&mut n).unwrap();
    let n: u128 = match n.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid number entered. Assuming n=0.");
            0
        }
    };

    if n == 0 {
        println!("Last fib: 0");
        return;
    }

    let mut num1 : u128 = 0;
    let mut num2 : u128 = 1;

    for _ in 0..n {
        let new_fib : u128 = num1 + num2;
        num1 = num2;
        num2 = new_fib;
    }

    println!("Last fib: {}", num2);
}