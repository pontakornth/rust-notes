use std::{env, process};

use documented::is_prime;

fn main() -> Result<(), &'static str> {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(raw_number) => {
            if let Ok(number) = raw_number.trim().parse::<i32>() {
                if is_prime(number) {
                    println!("{number} is a prime number!");
                } else {
                    println!("Unfortunately, {number} is not a prime number!");
                }
            } else {
                eprintln!("Invalid number is supplied!");
                process::exit(1);
            }
        }
        None => {
            eprintln!("No number supplied.");
            process::exit(1);
        }
    };
    Ok(())
}
