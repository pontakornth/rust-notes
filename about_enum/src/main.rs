use std::f64::consts::PI;
// This is how enum is defined.
enum TrafficColor {
    Red,
    Green,
    Blue,
}

// You can add a value to an enum as well.
enum IpAddress {
    V4(String),
    V6(String),
}

// You can put anything in an enum
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
}

// You can even implement method for an enum
impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle(radius) => radius * radius * PI,
            Shape::Rectangle(width, height) => width * height,
        }
    }
}

// I am going to use crypto because I am evil.
enum Coin {
    BitCoin,
    DogeCoin,
    SadCoin,
    TofuCoin,
}

fn main() {
    println!("Hello, world!");
    // Let's initialize an enum.
    let your_ip_address = IpAddress::V4(String::from("127.0.0.1"));
    let loopback = IpAddress::V6(String::from("::1"));

    // Option is so useful. Rust even has it in the prelude.
    let result_from_operation = some_operation_that_can_return_nothing(99);
    println!("{:?}", result_from_operation);

    let a_coin = Coin::TofuCoin;
    match a_coin {
        Coin::BitCoin => println!("This is a Bitcoin"),
        Coin::SadCoin => println!("Sad coin...T_T"),
        Coin::DogeCoin => println!("To the moon"),
        Coin::TofuCoin => println!("Some coin"),
    }
    
    // match must be exhaustive
    match a_coin {
        Coin::BitCoin => println!("This is a bitcoin"),
        // This expression match any coin
        // _ is convention for unused placeholder. If it is used, you can use any variable just
        // like other parts of Rust code.
        _ => println!("not a bitcoin")
    }

    match a_coin {
        Coin::BitCoin => println!("BitCoin!!!"),
        // () means do nothing.
        // It is Unit.
        _ => ()
    }
    
    // If you only care about a case, use `if let`
    let config_maximum = Some(88);
    if let Some(maximum) = config_maximum {
        println!("Configurated value is {}", maximum);
    } else {
        println!("config maximum value pls");
    }

    // If you use more than one condition, just use match.
}

fn some_operation_that_can_return_nothing(n: i32) -> Option<i32> {
    if n == 3 {
        Some(32)
    } else {
        None
    }
}
