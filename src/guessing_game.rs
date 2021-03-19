use std::io;

fn main() {
    println!("Guess the number!");
    println!("-----------------\n");

    println!("Input your guess: ");
    let mut guess = String::new(); //Create new string variable

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed the read line!");

    println!("You guessed: {}", guess);
}
