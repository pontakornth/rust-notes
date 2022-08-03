const THIS_IS_A_CONSTANT_WRYYYY: u32 = 80; // Compile time constant. It is always immutable.

fn main() {
    let x = 10; // Define a variable
    println!("The value of x is {x}");
    {
        let x = 99;
        println!("The value of x in this scope is {x}");
    }
    println!("The value of x outside the scope is unchanged. It is {x}.");
    // While you can indeed re-declare variable, it is easier just to use let.
    let mut changable = 190;
    changable = 90;
    // However, you cannot change type of variable. This is only possible with re-declaring ref.
    println!("Changable is changed. The value change from 190 to {changable}");
    println!("Hello, world!");

    // Division is floored.
    let floored_result: i32 = 2 / 3;
    // single char is single quote
    let a_char = 'a';
    // Not ascii
    let another_char = 'ℤ';
    // When a man uses emoji
    let heart_eyes_cat = '😻';

    // Tuple type is basically grouped together.
    let tup: (i32, u32, f64) = (12, 12, 3.3);
    // You can even destructure it.
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    // You can also use index
    let x2 = tup.0;
    println!("x is {x}, and x2 is same as x2 ({x2})");

    // Array is fixed
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // You can define a repeating array with this syntax
    // Equivalent: let repeated = [1,1,1,1,1];
    let repeated = [1; 5];

    // This is how you access an item of an array in Rust.
    let first_item_of_a = a[0];
    // It possible to make an out of bound access.
    // It will make Rust panics (runtime error) instead.
    // Uncomment to see.
    // let forty_two_item_of_a = a[41];
    empty_function();

    let result = add(99, 10);
    println!("The result is {result}");

    // Control flow
    // if is basically if. In Rust, you must use boolean only.
    if 2 == 3 {
        println!("Impossible");
    } else {
        println!("Possible");
    }

    // This is dumb. Just use 2 == 3
    let value_from_if = if 2 == 3 { true } else { false };
    let value_but_smarter = 2 == 3;

    loop {
        // Infinite loop. Use `break` to break out.
        break;
    }
    // just like if you can use loop for expression
    let mut counter = 0;
    let value_from_loop = loop {
        counter += 1;
        if counter > 99 {
            break counter;
        }
    };
    // Just like the preious code, this is dumb. It's just for demonstration purpose.

    // In case you have more than one nested loop, you can use label to braek out of a specific
    // loop. Otherwise, it would break the innermost loop.
    'outer_heaven: loop {
        println!("something like that!");
        let mut counter = 90;
        loop {
            println!("Counting to {counter}");
            counter += 1;
            if counter == 99 {
                break 'outer_heaven;
            }
        }
    }

    // If you want to use a loop with condition, just use while like in other languages.
    let mut counter = 0;
    while counter < 90 {
        // Old syntax.
        println!("Counter is {}", counter);
        counter += 1;
    }

    // Range for loop. It is like Python version of for-loop.
    // 1..10 is exclusive. 10 is not included.
    // 1..=10 is inclusive. 10 is included.
    for i in 1..10 {
        println!("The value of i is {i}");
    }

    // Since for loop in a collection simiar to Python.
    // You can use an array as well.
    let very_big_array = [1, 23, 4, 5, 23, 616, 61, 61, 62, 6];
    for item in very_big_array {
        println!("item is {item}");
    }
    for item in (1..100).rev() {
        println!("Reversed {item}");
    }
}

fn empty_function() {
    // This is a function without any functionality (pun intended).
    // Its purpose is to show Rust syntax of defining a function.
}

fn add(a: i32, b: i32) -> i32 {
    // This is a function with parameters. Anything passed is called an argument.
    // This is fully typed. I even annotated return type.
    // You don't need return statement. The last expression is used as the return value.
    // Don't use semi colon! It will not be returned.
    a + b
}
