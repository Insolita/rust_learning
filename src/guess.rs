use std::io;
use rand::Rng;

pub fn main() {
    let mut randomizer = rand::thread_rng();
    let my_number: u32 = randomizer.gen_range(1, 20);
    let mut try_counter: u8 = 1;
    let mut is_guessed: bool = false;

    println!("Guess what number I have thought");
    while try_counter <= 5 && !is_guessed {
        let mut guess = String::new();
        println!("Try # {}. Input your variant:", try_counter);
        io::stdin().read_line(&mut guess).expect("Failed to read line");

        let guess :u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if guess > my_number {
            println!("Too big!"); try_counter +=1;
        }else if guess < my_number {
            println!("Too small!"); try_counter +=1;
        }else {
            println!("Bingo!"); is_guessed = true;
        }
    }

    if !is_guessed {
        println!("Max tries reached. My number was {}", my_number);
    }

}
