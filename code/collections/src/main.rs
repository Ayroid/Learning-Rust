mod question_one;
use question_one::{median, mode};

fn main() {
    let mut list_of_integers = vec![1, 9, 8, 5, 2, 9];

    let median_answer: i32 = median(&mut list_of_integers);
    println!("Median of {:?} is {}", list_of_integers, median_answer);

    let mode_answer: i32 = mode(&list_of_integers);
    println!("Mode of {:?} is {}", list_of_integers, mode_answer);
}
