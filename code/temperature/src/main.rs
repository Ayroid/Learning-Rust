// User inputs the Number (f64) and Current Type (char)
// Based off the Type check using if clause and print the temperature

use std::io;

fn main() {
    println!("Enter temperature units");

    let mut temp = String::new();
    io::stdin()
        .read_line(&mut temp)
        .expect("Failed to read line");

    let temp: f64 = match temp.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
    };

    println!("Choose: c or f");

    let mut temp_type = String::new();
    io::stdin()
        .read_line(&mut temp_type)
        .expect("Failed to read temperate type");
    let temp_type: char = match temp_type.to_ascii_lowercase().trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid Input");
            return;
        }
    };

    match temp_type {
        'c' => {
            let fahrenheit = (temp * 9.0 / 5.0) + 32.0;
            println!("{}C = {}F", temp, fahrenheit);
        }
        'f' => {
            let celcius = (temp - 32.0) * 5.0 / 9.0;
            println!("{}F = {}C", temp, celcius);
        }
        _ => println!("Invalid input"),
    }
}
