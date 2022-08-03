const THIS_IS_A_CONSTANT_WRYYYY: u32 = 80; // Compile time constant. It is always immutable.

fn main() {
    let x = 10; // Define a variable
    println!("The value of x is {x}");
    {
        let x = 99;
        println!("The value of x in this scope is {x}");
    }
    println!("The value of x outside the scope is unchanged. It is {x}.");
    // While you can indeed re-declare variable, it is easier just to use let.
    let mut changable = 190;
    changable = 90;
    // However, you cannot change type of variable. This is only possible with re-declaring ref.
    println!("Changable is changed. The value change from 190 to {changable}");
    println!("Hello, world!");

    // Division is floored.
    let floored_result: i32 = 2 / 3;
    // single char is single quote
    let a_char = 'a';
    // Not ascii
    let another_char = 'ℤ';
    // When a man uses emoji
    let heart_eyes_cat = '😻';

    // Tuple type is basically grouped together.
    let tup: (i32, u32, f64) = (12, 12, 3.3);
    // You can even destructure it.
    let (x, y, z) = tup;
    println!("{x}, {y}, {z}");

    // You can also use index
    let x2 = tup.0;
    println!("x is {x}, and x2 is same as x2 ({x2})");

    // Array is fixed
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // You can define a repeating array with this syntax
    // Equivalent: let repeated = [1,1,1,1,1];
    let repeated = [1; 5];

    // This is how you access an item of an array in Rust.
    let first_item_of_a = a[0];
    // It possible to make an out of bound access.
    // It will make Rust panics (runtime error) instead.
    // Uncomment to see.
    // let forty_two_item_of_a = a[41];
    

    
}
