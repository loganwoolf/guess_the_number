use std::io;

fn main() {
    println!("Welcome to Guess the Number!");

    println!("Make your first guess: ");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read guess");

    println!("Your guess: {}", guess);
}
