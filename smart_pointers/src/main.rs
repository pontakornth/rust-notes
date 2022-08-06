// Smart Pointer = Data Structure that behaves like a pointer + addition
// In some cases, smart pointers own the data they have.

enum List {
    Con(i32, Box<List>),
    Nil,
}

enum RcList {
    RcCon(i32, Rc<RcList>),
    RcNil,
}

#[derive(Debug)]
enum RefList {
    RefCon(Rc<RefCell<i32>>, Rc<RefList>),
    RefNil,
}

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::cell::RefCell;
use std::{ops::Deref, rc::Rc};

use crate::List::{Con, Nil};
use crate::RcList::{RcCon, RcNil};
use crate::RefList::{RefCon, RefNil};

fn hello(a: &str) {
    println!("{a}");
}

struct WeirdPointer {
    data: String,
}

impl Drop for WeirdPointer {
    fn drop(&mut self) {
        println!("{} is dropped. I am sad now.", self.data);
    }
}

fn main() {
    println!("Hello, world!");
    // Box is basically data on heap.
    // Use when
    // - Type that size cannot known on compile time.
    // - Large amount of data but don't want to copy.
    // - Want data that implement certain trait.
    let b = Box::new(9);
    println!("b = {b}");
    // Let's make a rescursive type.
    let list = Con(1, Box::new(Con(9, Box::new(Nil))));

    // You can use Deref trait to make smart pointers look like a normal reference.
    // Let's take a look at MyBox for custom data strucuture.

    assert_eq!(9, *b);
    let g = MyBox::new(10);
    assert_eq!(10, *g);

    // Implicit Deref coersion means deref to another type.
    let why = MyBox::new(String::from("Haiya"));
    hello(&why);

    // Drop trait implementation
    {
        let s = WeirdPointer {
            data: String::from("weird"),
        };
    }

    // You can manually drop value using std::mem::drop. It is different from automatic dropping.

    // Rc is commonly used when the data can have multiple owner such as graph.
    // Uncomment three lines to show errors.
    // let a = Con(2, Box::new(Nil));
    // let b = Con(2, Box::new(a));
    // let c = Con(3, Box::new(a)); // problem here
    let a = Rc::new(RcCon(2, Rc::new(RcNil)));
    let b = RcCon(3, Rc::clone(&a));
    let c = RcCon(9, Rc::clone(&a));

    // Rc::clone only increases reference count.
    println!("{}", Rc::strong_count(&a));

    // In case that you need multiple owner with one mutable reference, use Rc with RefCell.
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(RefCon(Rc::clone(&value), Rc::new(RefNil)));

    let b = RefCon(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = RefCon(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}

// About weak Rc, please consult the book.
