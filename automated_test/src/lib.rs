// How to write tests
// 1. Set up any needed data or stater
// 2. Run code
// 3. Assert the result
//

#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width >= other.width && self.height > other.height
    }
    fn can_hold_other_way(&self, other: &Rectangle) -> bool {
        self.width >= other.height && self.height >= other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // Uncomment this to make test fails
    #[test]
    fn it_fails() {
        // panic!("Failure");
    }

    #[test]
    fn assertion() {
        // assert makes test panic if it is false.
        let larger = Rectangle {
            width: 99,
            height: 99,
        };
        let smaller = Rectangle {
            width: 12,
            height: 12,
        };
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn not_eq() {
        assert_ne!(3, 2);
    }

    // Note: assert_ne and assert_eq requires anything insides to have Debug trait because it
    // prints if the test fails.
    //

    #[test]
    fn custom_message() {
        // Uncomment to make test fails.
        // assert!(false, "It fails {}", 1);
    }

    // You can even test if something should panic
    #[test]
    #[should_panic]
    fn test_panic() {
        // Uncomment to make test fails.
        panic!("Yes");
    }

    // you can make test only pass if panics is from expected source
    const real_panic: &'static str = "This should happens";

    #[test]
    #[should_panic(expected = "{}", real_panic)]
    fn test_expected_panic() {
        // Uncomment to make test fails.
        // panic!("False");
        panic!("{}", real_panic);
    }

    // You can even use Result type instead of assertion.
    #[test]
    fn result_test() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Something weird"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        assert!(true);
    }
}
