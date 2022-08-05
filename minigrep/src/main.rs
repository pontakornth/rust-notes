use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();

    // 0 is the program name.
    let query = &args[1];
    let filename = &args[2];
    println!("Searching for {}", query);
    println!("In file {}", filename);
}
