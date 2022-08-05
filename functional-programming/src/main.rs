use std::{thread, time::Duration};

#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

fn square_only_list(rectangles: Vec<Rectangle>) -> Vec<Rectangle> {
    rectangles
        .into_iter()
        .filter(|r| r.width == r.height)
        .collect()
}

fn main() {
    println!("H&ello, world!");
    // Closures are anonymouse functions that can capture environment outside.
    let result_from_a_function: Option<&str> = Some("yes");
    // || "yes" is a closure.
    let real_result = result_from_a_function.unwrap_or_else(|| "yes");
    // Unlike functions, closures do not need type annotations.
    // However, we can do it if it is desired.
    let annotated = || {
        println!("Something is wrong");
    };
    let expensive_closure = |num: u32| -> u32 {
        println!("I am the slowness");
        thread::sleep(Duration::from_secs(0));
        num
    };
    let num = expensive_closure(99);
    println!("Finished expensive operations.");
    // Even closure does not need a type annotation, it can only means one type
    let identity = |x| x;
    // Uncomment one of them to see errors.
    let identity_number = identity(32);

    // There are three ways closures can capture values from environment.
    // 1. Taking parameters.
    // 2. Borrowing immutably.
    // 3. Taking ownership.
    //
    // Mutable borrow
    let mut a_list: Vec<i32> = vec![];
    let mut append_nine = || a_list.push(9);
    append_nine();
    append_nine();
    println!("{:?}", a_list);

    // This text is copied from Book of Rust.
    // Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion:
    //
    // FnOnce applies to closures that can be called at least once. All closures implement this trait, because all closures can be called.
    // If a closure moves captured values out of its body, then that closure only implements FnOnce and not any of the other Fn traits, because it can only be called once.
    //
    // FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
    //
    // Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values.
    // These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
    // Closures that don’t capture anything from their environment implement Fn.
    //

    // Closure used here implements FnMut
    let mut vec_rectangles = vec![
        Rectangle {
            width: 99,
            height: 10,
        },
        Rectangle {
            width: 9,
            height: 12,
        },
        Rectangle {
            width: 70,
            height: 13,
        },
    ];
    // Mutation!
    vec_rectangles.sort_by_key(|r| r.width);
    println!("{:?}", vec_rectangles);

    // Iterator allows us to performs task on a sequence of values.
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    // for val in v1_iter {
    //     println!("value is {val}");
    // }
    assert_eq!(Some(&1), v1_iter.next());
    assert_eq!(Some(&2), v1_iter.next());
    assert_eq!(Some(&3), v1_iter.next());
    assert_eq!(None, v1_iter.next());
    let other_v1_iter = v1.iter();
    // Type annotation is required.
    let summation: i32 = other_v1_iter.sum();
    println!("{summation}");

    // Note: Iterators do nothing unless they are used.
    let v2 = v1.iter().map(|x| x + 1);
    let squares = square_only_list(vec_rectangles);
    println!("Number of squares is {}", squares.len());
}
