use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("please input your guess.");

        let mut guess = String::new();

        std::io::stdin().read_line(&mut guess).expect("failed to read line");
        println!("you guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number){
            Ordering::Less => println!("go higher"),
            Ordering::Equal => { 
                println!("wow!, you got it");
                break;
            },
            Ordering::Greater => println!("go lower"),
        }
    }
}
