use std::error::Error;
use std::fs::File;
use std::io::stdin;

#[derive(PartialEq)]
enum NumberError {
    DivisionError,
}

fn divide(a: i32, b: i32) -> Result<i32, NumberError> {
    if b == 0 {
        return Err(NumberError::DivisionError);
    };

    Ok(a / b)
}

fn short_divide(a: i32, b: i32) -> Result<i32, NumberError> {
    // This is dumb. I only make it demonstrate ? operator.
    let result = divide(a, b)?; // result is i32. If there is an error, it will exit with error from divide function.
    Ok(result)
}

// ? can be used for option as well.
// Code from Book of Rust.
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
// NOTE: Since this lesson is about error handling, you may need to uncomment some of the code to
// see any error.
fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    // panic! stops program execution from unrecoverable error.
    // Let's make an error if you type 3 or invalid integer.
    println!("Don't type 3 or invalid integer.");
    let mut result: String = String::new();
    stdin().read_line(&mut result).expect("Can't readline");
    let result: u32 = result.trim().parse().expect("Invalid integer");
    if result == 3 {
        panic!("3 is bad.");
    }
    // use RUST_BACKTRACE=1 cargo run to backtrace step.
    //
    // Most errors are expected such as incorrect input.
    // That's why Rust has Result type. Result is Ok(T) or Err(E).
    // It is an enum.
    // Let's open a random file we don't create.
    // Uncomment to see an error.
    // let _unused_file = match File::open("some_weird_file_we_will_not_create.txt") {
    //     // Why did you create this?
    //     Ok(file) => file,
    //     Err(err) => panic!("Cannot open file {:?}", err)
    let magic_number: u32 = "32".parse().unwrap();
    // If you want a custom error message, use expect
    let magic_number: u32 = "99".parse().expect("A weird thing happens");

    // Progagating error is a fancy word for letting the caller handles the error instead of
    // handling it in the function.
    match divide(9, 0) {
        Ok(result) => println!("The universe breaks!"),
        Err(er) => {
            if er == NumberError::DivisionError {
                println!("Divide by zero");
            }
        }
    }

    // You can even use more shortcut with ? operator
    // See short_divide for detail
    
    Ok(())
}
