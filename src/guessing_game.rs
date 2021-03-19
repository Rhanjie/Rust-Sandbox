use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

pub fn main() {
    println!("Guess the number!");
    println!("-----------------\n");

    let secret_number = rand::thread_rng().gen_range(1..101);

    let mut attempts = 0;

    loop {
        print!("Input your guess: ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed the read line!");

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        attempts += 1;

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win after {} attempts!!", attempts);

                break;
            }
        }
    }
}
