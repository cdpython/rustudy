use rand::Rng;
use std::io;

fn main() {
    println!("Guessing Game!");
    let mut rng = rand::thread_rng();
    let mut random: u8 = rng.gen();
    random %= 101;
    // println!("range 0~100 value {}", random);
    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                println!("number only!");
                continue
            }
        };
        println!("guess: {}", guess);
        match guess.cmp(&random) {
            std::cmp::Ordering::Less => println!("Too Small!"),
            std::cmp::Ordering::Greater => println!("Too Big!"),
            std::cmp::Ordering::Equal => {
                println!("Coool! You Win!");
                break;
            }
        }
    }
}
