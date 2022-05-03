use std::io; // prelude
use std::cmp::Ordering;
use rand::Rng;

/**
 * cargo new 2.guess_number --name guess_number
 */
fn main() {
    let secret_number = rand::thread_rng().gen_range(10, 100); // rand number
    
    loop {
      println!("Guess Number!");
      let mut guess = String::new();
      io::stdin()
      .read_line(&mut guess)
      .expect("cannot read...");
      println!("Your input number is: {}", guess);

      // let guess:u32 = guess.trim().parse().expect("Please type a number!");

      let guess:u32 = match guess.trim().parse() {
        Ok(num) => num,
        Err(_) => continue,
      };

      match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => {
          println!("You win!");
          break;
        }
      }
    }
    println!("The secret number is: {}", secret_number);
}
