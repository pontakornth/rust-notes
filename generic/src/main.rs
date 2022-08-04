use crate::content::{Summary, Tweet};

mod content;
//
// Generic allows us to define a function without restricting to a very specific type.
//
fn swap_int_pair(a: (i32, i32)) -> (i32, i32) {
    (a.1, a.0)
}

fn swap_pair<T>(a: (T, T)) -> (T, T) {
    (a.1, a.0)
}

// You can restrict to a type having some trait.
fn larger<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

// You can also use it in struct definition
struct Point<T> {
    x: T,
    y: T,
}

// Also method
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// You can implement methods for a certain type as well.
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct MPoint<T, U> {
    x: T,
    y: U,
}

// You can even use a different type in method
impl<X1, Y1> MPoint<X1, Y1> {
    fn with_other_y<X2, Y2>(self, other: MPoint<X2, Y2>) -> MPoint<X1, Y2> {
        MPoint {
            x: self.x,
            y: other.y,
        }
    }
}

impl Summary for String {
    fn summarize(&self) -> String {
        self.clone()
    }
}

// In Rust, lifetime is about when the value is dropped.
// It is important when you are dealing with reference especially when you are returning a
// reference to something.
// &'a i32 is a reference to an i32 value with lifetime of a. (a is a convention.)
//
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    // Take string that lifetime at least 'a ( The smallest one).
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    println!("Hello, world!");
    let a_tuple = (3, 2);
    let b_tuple = swap_int_pair(a_tuple);
    let c_tuple = swap_pair(a_tuple);
    println!("b_tuple = {:?}, c_tuple = {:?}", b_tuple, c_tuple);

    // Use trait
    let a_tweet = Tweet {
        username: "pontakornth".to_string(),
        body: "Terraria is good".to_string(),
    };
    println!("{}", a_tweet.summarize());
    println!("{}", "something".to_string().summarize());

    // This won't work. Uncomment to see errors.
    let x = "Long logn".to_string();
    let result: &str;
    {
        let y = "longer than you".to_string();
        result = longest(x.as_str(), y.as_str());
        // result is dropped here as it tooks the shortest lifetime.
    }
    // println!("{result}");
    //
    // static lifetime indicates that it will live until the end of the program.
    // Some time error message says you should make it static. However, you should try to think
    // when the reference should ends instead.
    //
    // static lifeime is commonly used when the value must live the whole execution.
    let a_string: &'static str = "This will live until the end of the program";
}
