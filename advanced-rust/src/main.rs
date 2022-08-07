// NOTE: This module is unlikely to be completed. I don't even want to add unsafe code.
// What you can do in unsafe Rust (copied from the book)
// Dereference a raw pointer
//
// Call an unsafe function or method
// Access or modify a mutable static variable
// Implement an unsafe trait
// Access fields of unions
//
//
//

use std::error::Error;
use std::ops::Add;
use std::slice;

// From book.
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
unsafe fn dangerous_function() {
    println!("Please assume I am doing something evil!");
}

extern "C" {
    fn abs(input: i32) -> i32;
}

// Other languages can call Rust code as well. However, we should not mangle it.
// Mangling is turning readable name into unreadable glibberish that might summon demons. It is
// readable for other programming language but difficult for humans.

#[no_mangle]
pub extern "C" fn call_from_c() {
    println!("Print like an evil!");
}

// static variable is basically variable that have static lifetime.
static mut COUNTER: u32 = 999;
// reading or writing is unsafe.
fn adding_to_counter(a: u32) -> u32 {
    unsafe {
        COUNTER += a;
        COUNTER
    }
}

// you can also have unsafe trait as well.

// Type placeholder is different from generics.
//
struct Millimeters(i32);
struct Meters(i32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, rhs: Meters) -> Self::Output {
        let Meters(value) = rhs;
        let Millimeters(self_value) = self;
        Millimeters(self_value + value * 1000)
    }
}

// It's possible that you have implemented the same method with different trait.
//
struct Human<'a> {
    name: &'a str,
}

impl Human<'_> {
    fn speak(&self) {
        println!("I am a man named {}", self.name);
    }
}

trait Wizard {
    fn speak(&self) -> ();
}

impl Wizard for Human<'_> {
    fn speak(&self) -> () {
        println!("Olen velho.");
    }
}

// You can use type alias to make long type shorter.

type WeirdType = Box<dyn Error + 'static>;

// ! can means a type that never returns
fn paniku() -> ! {
    panic!("Hel");
}

// There is a special trait called Sized. It means the size is known at the compile time.

// There can be a function type as well.
fn take_function(input: i32, f: fn(i32) -> i32) -> i32 {
    f(input)
}

// When returning a function, it is dynamic.
// Make sure to wrap in a box.
fn return_function() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

// It's best to write a function that accept both a closure and function. In some case, it can only
// accept a function (C code does not have closure.).
// fn implements Fn, FnMut, and FnOnce

// Macro is basically programs a program to write a program.
// Macro can have any number of arguments. Just like println!.

// Declarative macro
#[macro_export]
macro_rules! my_vec {
    ($( $x:expr), *) => {
        {
        let mut temp_vec = Vec::new();
        $(
            temp_vec.push($x);
        )*
            temp_vec
        }
    };
}

// macro_export means it can be brought into scope.
// $x:expr means any expression given name x.
// $() indicates comman separator.

// Procedure macro.
// It acts like a function. It is for custom Derive. It's so complicated I don't think I will
// understand or have to write it at all. Please read the book.
fn main() {
    // unsafe function must be called in an unsafe block.
    // The entire unsafe function is an unsafe block.
    unsafe {
        dangerous_function();
    }
    println!("Hello, world!");
    unsafe {
        println!("abs(-3) == {}", abs(-3));
    }
    let a_human = Human { name: "Guy" };
    // Speak for human
    a_human.speak();
    // Even Human impl Wizard, we won't find a wizard speaking here.
    // Unless, you explicitly use a wizard
    Wizard::speak(&a_human);
    // or make them a wizard.
    let b: Box<dyn Wizard> = Box::new(a_human);
    b.speak();
    let a = my_vec![1, 2, 3];
}
