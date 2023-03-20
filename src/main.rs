// // use rand::Rng;
// // use std::cmp::Ordering;
// // use std::io;
// struct Test {
//     name: String,
//     active: bool,
//     age: u8,
//     in_game_name: String,
// }
// fn main() {
//     let mut sample: Test = Test {
//         name: String::from("Hazel"),
//         active: false,
//         age: 10,
//         in_game_name: String::from("hoomanBean"),
//     };
//     //if i comment this line line number 22 throw error as in rust we use ownership and borrow model and
//     // in line 23 i am giving ownership of data from sample  to sample 2
//     // hence the error
//     let sample2: Test = Test {
//         name: String::from("Ankit"),
//         in_game_name: String::from("kor0"),
//         ..sample
//     };
//     println!(
//         "Struct value  name {} and is she active {} in gama name {}",
//         sample.name, sample.active, sample.in_game_name
//     );
//     sample.name = String::from("Dino");
//     sample.age = 12;
//     println!(
//         "Struct value  name {} and is he active {} with age  {} in game name {}",
//         sample2.name, sample2.active, sample2.age, sample2.in_game_name
//     );
//     println!("Struct value  name {}", sample.in_game_name);
//     // Guess game
//     // println!("Guess the number!");
//     // let sec_num = rand::thread_rng().gen_range(1..=100);
//     // loop {
//     //     println!("Please input your guess. {sec_num}");
//     //     let mut guess = String::new();
//     //     io::stdin()
//     //         .read_line(&mut guess)
//     //         .expect("Failed to read line");
//     //     // // let guess: u32 = guess.trim().parse().expect("Please type a number!");
//     //     let guess: u32 = match guess.trim().parse() {
//     //         Ok(num) => num,
//     //         Err(_) => continue,
//     //     };
//     //     println!("You guessed: {guess}");
//     //     println!("guess is {guess}");
//     //     match guess.cmp(&sec_num) {
//     //         Ordering::Less => println!("Too small!"),
//     //         Ordering::Greater => println!("Too big!"),
//     //         Ordering::Equal => {
//     //             println!("You win!");
//     //             break;
//     //         }
//     //     }
//     // }
// }
// // fn main() {
// //     for number in (1..4).rev() {
// //         println!("{number}!");
// //     }
// //     println!("LIFTOFF!!!");
// // }
// //cargo doc --open -> to get doc
// //cargo new pro_name
// //cargo run
// //cargo build
// //Kinda like Arrow function
// // fn five() -> i32 {
// //     5
// // }
// // fn main() {
// //     let x = five();
// //     println!("The value of x is: {x}");
// // }
// //Expression will execute
// // fn plus_one(x: i32) -> i32 {
// //     x + 1
// // }
// //by adding ; it became Statement and will not  execute
// // fn plus_one(x: i32) -> i32 {
// //     x + 1
// // }
// //OwnerShip Rule
// //1-> Each val has a variable that called its owner
// //2-> can only be 1 owner at time
// //3-> owner out of scope val get dropped
// // & gives reference instead of ownership knows as browning and immutable can make mutabale by adding mut  and &mut but can't borrow mpre than once
// // Data race ->
// //Dangle reference
// //////////////////////////////////////////////////////
// //cant disply struct with simple println as it used a formatting cale display which is usefullfor primitive valey like int or string bit for value like struct ---> Let’s try it! The println! macro call will now look like println!("rect1 is {:?}", rect1);. Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.
// //ERRor with struct in printlin
// // = help: the trait `std::fmt::Display` is not implemented for `Rectangle`
// // = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
// //Affer adding {:?} stulll get erroe bcz
// //Error -> error[E0277]: `Rectangle` doesn't implement `Debug`
// //Reasion -> Rust does include functionality to print out debugging information, but we have to explicitly opt in to make that functionality available for our struct. To do that, we add the outer attribute #[derive(Debug)] just before the struct definition, as shown in Listing 5-12.
// //To solve add
// //#[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }
// //But
// // Nice! It’s not the prettiest output, but it shows the values of all the fields for this instance, which would definitely help during debugging. When we have larger structs, it’s useful to have output that’s a bit easier to read; in those cases, we can use {:#?} instead of {:?} in the println! string. In this example, using the {:#?} style will output the following:
// //Also
// // Another way to print out a value using the Debug format is to use the dbg! macro, which takes ownership of an expression (as opposed to println!, which takes a reference), prints the file and line number of where that dbg! macro call occurs in your code along with the resultant value of that expression, and returns ownership of the value.
// // Note: Calling the dbg! macro prints to the standard error console stream (stderr), as opposed to println!, which prints to the standard output console stream (stdout). We’ll talk more about stderr and stdout in the “Writing Error Messages to Standard Error Instead of Standard Output” section in Chapter 12.
// // //EXample
// // #[derive(Debug)]
// // struct Rectangle {
// //     width: u32,
// //     height: u32,
// // }
// // fn main() {
// //     let scale = 2;
// //     let rect1 = Rectangle {
// //         width: dbg!(30 * scale),
// //         height: 50,
// //     };
// //     dbg!(&rect1);
// // }
struct Reactangle{
    width:u8,
    height:u8
}
impl Reactangle {
    fn area(&self)-> u32{
        self.width * self.height;
    }
}
fn main(){
 const  newArea =Reactangle{
    width:4,
    height:4,
 };
}