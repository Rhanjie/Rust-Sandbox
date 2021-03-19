mod guessing_game;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::io::Write;

fn main() {
    println!("-------------------------------------------------------");
    println!("----------------------[ Sandbox ]----------------------");
    println!("-------------------------------------------------------\n");

    guessing_game::main();
}
