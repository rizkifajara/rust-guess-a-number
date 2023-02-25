use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::process::Command;

fn main() {
    println!("Guess the number! (1-1000)");

    let secret_number = rand::thread_rng().gen_range(1..=1000);

    let mut live = 10;

    loop  {

        println!("You have {live} guess left.");

        if live == 0 {
            println!("You lose!");
            break;
        }

        println!("Please input your guess: ");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                live -= 1;
                continue;
            },
        };

        // clear terminal after guessing
        if cfg!(target_os = "windows") {

            Command::new("cls").status().unwrap();

        } else {

            Command::new("clear").status().unwrap();

        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                println!("Too small!");
                live -= 1;
            },
            Ordering::Greater => {
                println!("Too big!");
                live -= 1;
            },
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }

    }
}
