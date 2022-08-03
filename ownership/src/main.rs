fn main() {
    // Ownership rule's
    // 1. Each value in Rust has an owner.
    // 2. There can be only one owner at a time.
    // 3. When the owner goes out of scope, the value will be dropped.

    let s = "hello"; // s is in main
    {
        let s = "hello"; // This s is in another scope.
    } // The inner s is dropped here.

    // Since most of values used in previous lessons are on the stack.
    // The size is known. It is trivial to copy thing.
    // We will focus on things on the heap such as a String.

    let mut s = String::from("Hello"); // This is a heap string.
    s.push_str(", world"); // This can be mutated.
    println!("{s}");
    println!("Hello, world!");
    // This is not copying.
    let s1 = String::from("Hello");
    let s2 = s1;
    // s1 and s2 are both a pointer pointing to a string "Hello".
    // Use book of rust for image.
    // Copying is expensive. If the string is large, we will run into performance problem.
    //
    // The problem is if both s1 and s2 are out of scope. They both will be freed.
    // Since they point to the same location, DOUBLE FREE.
    //
    // Rust make s1 invalid to prevent this problem.
    // println!("{s1}"); Uncomment this
    println!("{s2}");
    // However, if deep copying is desired. Use clone()
    let s3 = s2.clone(); // s2 is valid
    println!("s2 is {s2}, s3 is {s3}");
    // If the value is stack only, it is fine.
    let x = 90;
    let y = x;
    println!("x is {x}, and y is {y}");
    let a_string = String::from("This is a string.");
    take_ownership(a_string);
    // a_string is moved into scope of the function.
    // It is dropped before this line.
    // Try to use it.
    // println!("{a_string}"); Uncomment this

    // For a stack based value such as i32, it is copied.

    // A function can also give ownership
    let function_result = give_ownership();
    // The string returned from give_ownership fucntion is dropped when the main function is
    // completed..
    let (same_string, length) = calculate_length(function_result);

    // You can see it is tedious. Let's use reference.
    let length = calculate_length_borrow(&same_string);
    // You can still use same_string here because is is not dropped.
    // However, you cannot mutate it. It is not mutable.
    // Reference is mutable by default.
    let mut s = String::from("Hello");
    world_the_string(&mut s);
    println!("The value of world-ed string is {s}");

    // One rule about mutable reference is you can only have one mutable reference at a time.
    let mut_s = &mut s;
    // It is not obvious until you use it.
    // Uncomment two lines below this.
    // let another_mut_s = &mut s;
    // println!("{mut_s}, {another_mut_s}");

    // you can use {} to create another scope. You cannot use both of them at the same time anyway.
    // However, you can use immutable reference many times as you want.
    let ref_to_s = &s;
    let another_ref_to_s = &s;
    let yet_anoter_ref_to_s = &s;
    println!("{ref_to_s}, {another_ref_to_s}, {yet_anoter_ref_to_s}"); // It's fine.

    // Slice refers to a portion of contagious elements in an array.
    // It is a kind of reference. It does not have ownership.
    let hello_world = String::from("Hello, World");
    // Both of them are slices of hello_world.
    let hello = &hello_world[..5]; // Rust range syntax allows dropping 0.
    let world = &hello_world[6..11];
    let another_world = &hello_world[6..];
    // Note: In this file, only ASCII will be used.
    // Slice must end at a valid byte of string. Multi-bytes string will cause an error.
    //
    // See first_wordl
    let sentence = String::from("I don't like JavaScript.");
    let word = first_word(&sentence);
    println!("The first word is {word}");

    // String literals are slice. That's why we cannot mutate &str.
    let a_slice = "a slice here";

    // They are the same.
    let first_word = first_word_but_slice(a_slice);
    let first_word = first_word_but_slice(&a_slice);
    let first_word = first_word_but_slice(&a_slice[..]);

    let a_string = String::from("a string here");
    // &a_string is a slice. a_string is not.
    let first_word = first_word_but_slice(&a_string);

}

fn take_ownership(some_string: String) {
    println!("{some_string}");
    // some_string is dropped here.
}

fn give_ownership() -> String {
    let take_me_pls = String::from("Pls take me");
    take_me_pls
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_borrow(s: &String) -> usize {
    s.len()
}

fn world_the_string(s: &mut String) {
    s.push_str(", world");
}

// Uncomment to show errors.
// s is dropped when the fuction ends.
// The reference will point to nowhere.
// fn dangling_pointer_example() -> &String {
//     let s = String::from("This");
//     &s
// }
//

fn first_word_length(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i; // Early return use this keyword.
        }
    }
    s.len()
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // Drop everything.
}

fn first_word_but_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..] // Drop everything.
}
