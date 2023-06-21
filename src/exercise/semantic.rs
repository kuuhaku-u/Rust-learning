// move_semantics1.rs
//
// Execute `rustlings hint move_semantics1` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DONE
fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DONE
fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0.clone());
    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);
    vec1.push(88);
    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
}
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
// move_semantics3.rs
//
// Make me compile without adding new lines-- just changing existing lines! (no
// lines with multiple semicolons necessary!)
//
// Execute `rustlings hint move_semantics3` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DONE
fn main() {
    let vec0 = Vec::new();
    let mut vec1 = fill_vec(vec0);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
fn fill_vec(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DONE
fn main() {
    // let vec0 = Vec::new();
    let mut vec1 = fill_vec();
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
// move_semantics4.rs
//
// Refactor this code so that instead of passing `vec0` into the `fill_vec`
// function, the Vector gets created in the function itself and passed back to
// the main function.
//
// Execute `rustlings hint move_semantics4` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DNE
fn main() {
    // let vec0 = Vec::new();
    let mut vec1 = fill_vec();
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}
// `fill_vec()` no longer takes `vec: Vec<i32>` as argument
fn fill_vec() -> Vec<i32> {
    let mut vec = Vec::new();
    vec.push(22);
    vec.push(44);
    vec.push(66);
    vec
}
// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DONE
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
// move_semantics6.rs
//
// You can't change anything except adding or removing references.
//
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand
// for a hint.
// I AM NOT DONE
fn main() {
    let data = "Rust is great!".to_string();
    get_char(&data);
    string_uppercase(&data);
}
// Should not take ownership
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}
// Should take ownership
fn string_uppercase(data: &String) {
    let data = data.to_uppercase();
    println!("{}", data);
}
