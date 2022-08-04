use std::collections::HashMap;

enum Spreadsheet {
    Int(i32),
    Float(f32),
    Double(f64),
    Text(String),
}
fn main() {
    // This is a vector that contains things similar to an array.
    let v = vec![1, 3, 4];

    // Vector can be updated if it is mutable.
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    println!("Hello, world!");
    {
        let a = vec![4, 5, 6];
    } // a is dropped here. It is just like any struct.
      // The content is also dropped. Be careful when you are adding reference to a vector.
      // You can access an item using index just like an array.
    println!("&v[0] == {}", &v[0]);
    // It will panic if it is out of bound just like array.
    match v.get(4) {
        Some(x) => println!("&v[4] is {}", x),
        None => println!("There is nothing."),
    }

    // You can also iterate over a vector.
    for item in &v {
        println!("{}", item);
    }

    // Of course you can iterate and mutate as well.
    for item in &mut v {
        *item += 90;
        println!("{item}");
    }

    // You can use enum to store multiple types.
    let sheet = vec![Spreadsheet::Int(35), Spreadsheet::Float(32.3)];

    // Let's talk about String.
    // String is basically characters. Rust encodes String in UTF-8.

    // Create a string
    let a_string = String::from("Evil");
    let a_string = "Evil".to_string();

    // You can insert other languages as well.
    let thai = String::from("ไก่");

    // You can mutate string if it is mutable.
    let mut mutable_string = String::from("");
    mutable_string.push_str("so");

    let good = String::from("holy");
    mutable_string = thai + &good;
    // Thai is moved here. It is invalid now.
    // Good is still valid.

    // You can also concat text using format! macro.
    // The format is like println! macro.
    mutable_string = format!("{}{}", mutable_string, good);

    // You cannot index string. However, you can slice string as long as it ends in a valid UTF-8
    // encoding. The reason behind this is some characters take more than 1 byte.
    // separating graphenes are not in the standard library. Use crates.io to find suitable crate
    // instead.
    //
    // You can iterates over chars or bytes. Use whatever makes sense for the operation.
    //
    for (index, char) in mutable_string.chars().enumerate() {
        println!("Char at {index} is {char}");
    }
    mutable_string.push_str("");

    // There is a HashMap to be used. It's like a dictionary.
    let mut team_score: HashMap<String, u32> = HashMap::new();
    team_score.insert(String::from("Red"), 0);
    team_score.insert(String::from("Green"), 1);

    // If the data type has no Copy trait, the ownership is moved into the hash map.
    // Otherwise, it just be copied.

    // You can use collect to create a new hashmap.
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();

    // You can get value using get() method
    if let Some(red_score) = team_score.get(&String::from("Red")) {
        println!("Red score is {red_score}");
    }

    // Checking if any value then insert is tedious.
    // Use or_insert.
    // If Red has no score, insert 99.
    team_score.entry(String::from("Red")).or_insert(99);

    // entry is a reference too. You can update it from old value.
    let green_score = team_score.entry(String::from("green")).or_insert(0);
    *green_score += 10;
}
