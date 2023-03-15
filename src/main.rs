use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("Guess the number!");
    let sec_num = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Please input your guess. {sec_num}");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        // // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");
        println!("guess is {guess}");
        match guess.cmp(&sec_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
//cargo doc --open -> to get doc
//cargo new pro_name
//cargo run
//cargo build
