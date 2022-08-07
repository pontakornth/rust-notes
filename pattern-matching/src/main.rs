//
#[derive(Debug)]
enum Command {
    Move { x: i32, y: i32 },
    ChangeColor(i32, i32, i32),
}

// OOP part is skipped as there is not enough code to make its own lesson.
// It's better to be read at the book.
fn main() {
    println!("Hello, world!");
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, &str> = Ok(9);
    // if let is basically if
    if let Some(color) = favorite_color {
        println!("My favorite color is {color}");
    } else if is_tuesday {
        println!("Today is Tuesday.");
    } else if let Ok(my_age) = age {
        println!("I am {my_age} year(s) old.");
    }

    // Yes, you can even use while let.
    let mut stack = Vec::new();
    stack.push("c");
    stack.push("b");
    stack.push("a");
    while let Some(top) = stack.pop() {
        println!("{top}");
    }

    // for also takes pettern
    let mut v = Vec::new();
    v.push(123);
    v.push(9);
    v.push(10);
    for (index, value) in v.iter().enumerate() {
        println!("Index: {index}, Value: {value}");
    }
    // if let can use patterns. `let` can also use pattern.
    let old = (12, 9);
    let new_swap = swap_tuple(&old);
    println!(
        "Old ({}, {}), New ({}, {})",
        old.0, old.1, new_swap.0, new_swap.1
    );

    // There are two types of pattern matching.
    // Irrefutable - Cannot fail
    // Refutable - Can fail
    // Rust will warn you if you try to `if let` irrefutable patterns
    // and error if you try to do unconditional refutable pattern.

    // You can match literal
    let x = 9;
    match x {
        1 => println!("Wrong"),
        2 => println!("Cad"),
        3 => println!("Hel no"),
        _ => println!("Hahahahahahahaha"),
    }

    // Multiple pattern
    let y = 10;
    match y {
        1 | 2 => println!("One or two"),
        10 | 3 => println!("Ten or three"),
        _ => println!("Something else"),
    }

    // Using same name as field is so common so it's possible.
    let command = Command::Move { x: 2, y: 9 };
    match &command {
        Command::Move { x, y } => println!("Move to ({x}, {y}) now!"),
        cmd => println!("command is {:?}", cmd),
    }

    // In pattern, _ is not bound to any variable.
    // So you can do this.
    match &command {
        Command::Move { x: _, y: _ } => println!("Move!"),
        _ => (),
    };

    // .. is for ignored the rest of value.
    match &command {
        Command::Move { x, .. } => println!("Move x = {x}"),
        _ => (),
    }

    // (..,last)
    // (first, ..)
    // NOT (.., second, ..) too ambiguous
    // You can use @ to bind value that match pattern. It is used full when you are using range.
    let x = 12;
    match x {
        1 => println!("One"),
        j @ 1..=99 => println!("x is in 1 to 98 {j}"),
        _ => println!("somewhere else"),
    }
}

// Even function can use Pattern.
fn swap_tuple(&(x, y): &(i32, i32)) -> (i32, i32) {
    (y, x)
}
