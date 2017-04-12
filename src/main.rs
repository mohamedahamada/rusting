extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main(){
    println!("Do you Rust ?");
    println!("Start guessing ...");

    let mut guess = String::new();

    let secret = rand::thread_rng().gen_range(1, 101);

    println!("The secret number is: {}", secret);

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("Your guess is {}", guess);

}