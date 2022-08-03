// This is how you define a struct
#[derive(Debug)]
struct User {
    active: bool,
    // We cannot use &str.
    username: String,
    email: String,
    sign_in_count: u64,
}

/// Build a user with default setting.
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 0,
    }
}

// Tuple Struct
struct Color(u32, u32, u32); // RGB
struct Point(i32, i32);

// Unit struct
// Usually used for implement a trait but no data.
// We will just skip it for now.
struct AlwaysEqual;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct Circle {
    radius: u32,
}

// Let's define a method for a rectangle
impl Rectangle {
    // &self is used because it should not take the ownership.
    // self is used when you transform into another thing and you don't need the old one anymore.
    // &self is an alias for self: &Self
    // Self is an alias for the type you are implementing method
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn can_hold(&self, another: &Rectangle) -> bool {
        // We should not take ownership. We just compare.
        self.width >= another.width && self.height > another.height
    }

    // Called: Rectangle::square
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

// Switching width and height is not a problem for this functions.
// Imagine drwing a rectangle with height and width swapped.
fn area_rectangle(width: u32, height: u32) -> u32 {
    width * height
}

/// Draw a filled square
fn draw_rectangle(width: u32, height: u32) {
    for _ in 0..height {
        for _ in 0..width {
            print!("#");
        }
        print!("\n");
    }
}

fn draw_rectangle_struct(rectangle: &Rectangle) {
    draw_rectangle(rectangle.width, rectangle.height);
}

fn main() {
    // Creating a struct.
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("some@email.com"),
        sign_in_count: 899,
    };
    println!("Hello, world!");
    let user2 = build_user(String::from("most@email.com"), String::from("most"));
    // More complex expression cannot use the new syntax. {variable}
    println!("{}", user2.email);

    // This is how to create a struct from an existing struct.
    let user1_but_signed_in_once = User {
        sign_in_count: 1,
        ..user1
    };
    // Also, user1 is invalid because the ownership is moved.
    // The sign_in_count and active are priminitive so they may be used again.
    println!("{}", user1.active);
    // Uncomment to see an error
    // println!("{}", user1.email);
    let a_rectangle = Rectangle {
        width: 6,
        height: 3,
    };
    draw_rectangle(a_rectangle.width, a_rectangle.height);
    println!("");
    draw_rectangle_struct(&a_rectangle);
    // Trait debug allow us to print the rectangle
    println!("{:?}", &a_rectangle);

    // Use method
    println!("The area of a_rectangle is {}", a_rectangle.area());

    let a_square = Rectangle::square(5);
    draw_rectangle_struct(&a_square);
}
