use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("the secret number is {secret_number}");

    println!("please input your guess.");

    let mut guess = String::new();

    std::io::stdin().read_line(&mut guess).expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("you guessed: {}", guess);

    match guess.cmp(&secret_number){
        Ordering::Less => println!("go higher"),
        Ordering::Equal => println!("wow!, you got it"),
        Ordering::Greater => println!("go lower"),
    }
}
