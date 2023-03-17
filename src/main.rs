// use rand::Rng;
// use std::cmp::Ordering;
// use std::io;
struct Test {
    name: String,
    active: bool,
    age: u8,
    in_game_name: String,
}
fn main() {
    let mut sample: Test = Test {
        name: String::from("Hazel"),
        active: false,
        age: 10,
        in_game_name: String::from("hoomanBean"),
    };
    //if i comment this line line number 22 throw error as in rust we use ownership and borrow model and
    // in line 23 i am giving ownership of data from sample  to sample 2
    // hence the error
    let sample2: Test = Test {
        name: String::from("Ankit"),
        in_game_name: String::from("kor0"),
        ..sample
    };
    println!(
        "Struct value  name {} and is she active {} in gama name {}",
        sample.name, sample.active, sample.in_game_name
    );
    sample.name = String::from("Dino");
    sample.age = 12;
    println!(
        "Struct value  name {} and is he active {} with age  {} in game name {}",
        sample2.name, sample2.active, sample2.age, sample2.in_game_name
    );
    println!("Struct value  name {}", sample.in_game_name);
    // Guess game
    // println!("Guess the number!");
    // let sec_num = rand::thread_rng().gen_range(1..=100);
    // loop {
    //     println!("Please input your guess. {sec_num}");
    //     let mut guess = String::new();
    //     io::stdin()
    //         .read_line(&mut guess)
    //         .expect("Failed to read line");
    //     // // let guess: u32 = guess.trim().parse().expect("Please type a number!");
    //     let guess: u32 = match guess.trim().parse() {
    //         Ok(num) => num,
    //         Err(_) => continue,
    //     };
    //     println!("You guessed: {guess}");
    //     println!("guess is {guess}");
    //     match guess.cmp(&sec_num) {
    //         Ordering::Less => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal => {
    //             println!("You win!");
    //             break;
    //         }
    //     }
    // }
}
// fn main() {
//     for number in (1..4).rev() {
//         println!("{number}!");
//     }
//     println!("LIFTOFF!!!");
// }
//cargo doc --open -> to get doc
//cargo new pro_name
//cargo run
//cargo build
//Kinda like Arrow function
// fn five() -> i32 {
//     5
// }
// fn main() {
//     let x = five();
//     println!("The value of x is: {x}");
// }
//Expression will execute
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
//by adding ; it became Statement and will not  execute
// fn plus_one(x: i32) -> i32 {
//     x + 1
// }
//OwnerShip Rule
//1-> Each val has a variable that called its owner
//2-> can only be 1 owner at time
//3-> owner out of scope val get dropped
// & gives reference instead of ownership knows as browning and immutable can make mutabale by adding mut  and &mut but can't borrow mpre than once
// Data race ->
//Dangle reference
