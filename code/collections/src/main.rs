use std::io;

mod question_one;
mod question_two;
use question_one::{median, mode};
use question_two::pig_latin;
fn main() {
    let mut list_of_integers = vec![1, 9, 8, 5, 2, 9];

    let median_answer: i32 = median(&mut list_of_integers);
    println!("Median of {:?} is {}", list_of_integers, median_answer);

    let mode_answer: i32 = mode(&list_of_integers);
    println!("Mode of {:?} is {}", list_of_integers, mode_answer);

    let mut input_string = String::new();
    match io::stdin().read_line(&mut input_string) {
        Ok(_) => (),
        Err(_) => eprintln!("Failed to read line"),
    }

    let pig_latin_string = pig_latin(&input_string);
    println!("Pig Latin of '{}' is '{}'", input_string.trim(), pig_latin_string);

}
