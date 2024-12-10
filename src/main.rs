use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);

    print!("The secret number is: {secret_number}");

    println!("\nPlease input your guess.");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
    let guess: u32 = guess.trim().parse().expect("Please this has to be a number");
    match guess.cmp(&secret_number) {
        Ordering::Less => print!("Too Small!"),
        Ordering::Greater=>print!("Too big!"),
        Ordering::Equal => print!("You win!"),
    }

}
