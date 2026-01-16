use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess a number b/w 1-100");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    // println!("The Secret Number is : {secret_num}");
    loop {
        println!("Enter your guess. ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Unable to read line");
        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You entered the number: {guess}");

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
