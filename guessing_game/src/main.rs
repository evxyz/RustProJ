use std::io;

fn main() {
    println!("Guess A Number Please!");
    println!("Please add your guess!");
// some mutable var 
    let mut guess = String::new(); 
// note end of line without colon 
// .expect is a  Result type 
// result types are enumerations.
    io::stdin().read_line(&mut guess)
        .expect("Failed to Read line");
    println!("you gessed: {}", guess);
}
