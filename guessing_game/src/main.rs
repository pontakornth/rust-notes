use rand::Rng;
use std::cmp::Ordering;
use std::io; // Bring standard library from prelude. Check out Rust's Prelude

// This is the entry point of the program. Everything is run from here.
fn main() {
    println!("Guess a number");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    // If we print magic number, we would lose fun. Just don't print it. Easy.
    // println!("The magic number is {secret_number}");
    println!("Please input your guess");

    loop {
        // io::stdin() return Stdin that handles all the input from stdin.
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess) // read_line returns a result that you need to handle
            .expect("Failed to read line"); // If something happens, the panic message is indicated here.
                                            //
                                            // Rust is strong typed so you can't just match a number with a string.
                                            // Try remove it and see the error. It should be pretty obvious if you have any fancy IDE or
                                            // text editor.
        let guess: u32 = match guess.trim().parse() {
            Ok(n) => n,
            Err(_) => continue,
        };
        println!("Your guessed is {guess}");
        // loop is basically infinite loop.
        // In other languages, you would call while true or something like that.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You got it right");
                break; // Break out of the loop
            }
        }
    }
}
