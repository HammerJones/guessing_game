use rand::Rng;
use std::io;
use std::cmp::Ordering;

fn take_guess() -> i32 {
    println!("Please enter a number between 0 & 100:\n");

    let mut guess: String = String::new();
    
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("\n");

    guess.trim().parse::<i32>().unwrap()
}

fn gen_number() -> i32 {
    let mut rng = rand::thread_rng();
    let number: i32 = rng.gen_range(0..100);

    number
}

fn compare_results(number: i32) {
    let mut is_equal: bool = false;

    while !is_equal {
        match take_guess().cmp(&number) {
            Ordering::Less => println!("Too low!\n"),
            Ordering::Greater => println!("Too high!\n"),
            Ordering::Equal => {
                println!("That's right!!");
                is_equal = true;
            },
        }
    }
}

fn main() {
    println!("Welcome to the guessing game!!!\n");

    compare_results(gen_number());
}