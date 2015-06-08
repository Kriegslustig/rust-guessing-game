extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
      println!("Guess the number!");
      let mut guess = String::new();
      io::stdin().read_line(&mut guess)
        .ok()
        .expect("Failed to read your guess");

      let guess: i32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Please pass a number");
          continue;
        },
      };

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("To small."),
        Ordering::Greater => println!("To big."),
        Ordering::Equal => {
          println!("correct");
          break;
        },
      }
    }
}
