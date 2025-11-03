use std::io;
use rand::{thread_rng, Rng};

fn main() {
    let mut start_range = String::new();
    let mut end_range = String::new();

    println!("Start range:\n ");
    io::stdin().read_line(&mut start_range).unwrap();
    println!("End range: ");
    io::stdin().read_line(&mut end_range).unwrap();

    let start_range: i32 = start_range.trim().parse().unwrap();
    let end_range: i32 = end_range.trim().parse().unwrap();

    let random_number = thread_rng().gen_range(start_range..end_range);

    let mut cnt : u32 = 0;

    loop {
        println!("Guess the number!: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();
        let guess: i32 = guess.trim().parse().unwrap();


        if guess == random_number {
            println!("You win!");
            cnt += 1;
            break
        }
        else {
            println!("Try again!");
            cnt += 1;
        }
    }

    println!("{:?} was the answer. You guessed {} times.", random_number, cnt);
}