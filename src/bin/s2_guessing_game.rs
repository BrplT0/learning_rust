use std::io;
use rand::{thread_rng, Rng};
use std::cmp::Ordering;

fn main() {
    let mut start_range = String::new();
    let mut end_range = String::new();

    println!("Start range:\n ");
    io::stdin().read_line(&mut start_range).unwrap();
    println!("End range: ");
    io::stdin().read_line(&mut end_range).unwrap();

    let start_range: i32 = start_range.trim().parse().unwrap_or(1);
    let end_range: i32 = end_range.trim().parse().unwrap_or(100);

    let random_number = thread_rng().gen_range(start_range..end_range+1);

    let mut cnt : u32 = 0;
    println!("Starting number: {}", start_range);
    println!("Ending number: {}", end_range);

    loop {
        println!("Guess the number!: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: i32 = guess.trim().parse().unwrap_or_else(|_| 1);

        match guess.cmp(&random_number) {
            Ordering::Less => {
                println!("Too small!");
                cnt += 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                cnt += 1;
            },
            Ordering::Equal => {
                cnt += 1;
                println!("You win!");
                break;
            }
        }
    }
    println!("{:?} was the answer. You guessed {} times.", random_number, cnt);
}