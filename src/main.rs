use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!");

    let secret_num = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    loop {
        println!("input your guess: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("errorrrrrrrrrr");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => {
                if num < 1 || num > 100 {
                    println!("please input a number in the range 1 - 100");
                    continue;
                }
                attempts += 1;
                num
            }
            Err(_) => {
                println!("please enter a valid number");
                continue;
            }
        };

        println!("you guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("that's actually small"),
            Ordering::Greater => println!("that's actually big"),
            Ordering::Equal => {
                println!("u got that right! it took you {} attempts", attempts);
                println!("congrats!!");

                break;
            }
        }
    }
}
