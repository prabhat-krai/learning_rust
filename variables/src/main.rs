fn main() {
    // this is a comment
    //constants
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    // mutabilty
    let mut x = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    //shadowing

    let y = 5;
    println!("The value of x is {}", y);
    let y = 6;
    println!("The value of x is {}", y);

    // scalar types
    // A scalar type represents a single value.
    // Rust has four primary scalar types: integers, floating-point numbers, Booleans, and characters.

    //Integers
    // i -> Signed Integer (can be negative)
    // u -> Unsigned Integer (positive)

    // Floating-Point
    // Rust’s floating-point types are f32 and f64, which are 32 bits and 64 bits in size

    // Boolean
    // let f: bool = false;
    // let t = true;

    // Character
    // defined in single quotes, while String is defined with double quotes
    // 4 bytes in size

    // Compound Type
    // Compound types can group multiple values into one type.
    // Rust has two primitive compound types: tuples and arrays.

    // Tuple
    // A tuple is a general way of grouping together a number of values with a variety
    // of types into one compound type.
    // Tuples have a fixed length: once declared, they cannot grow or shrink in size.
    // let tup: (i32, f64, u8) = (500, 6.4, 1);

    // Destructuring in Tuple
    // let tup = (500, 6.4, 1);

    // let (x, y, z) = tup;

    // Use period to access values in tuple
    // let five_hundred = x.0;

    // Array
    // Unlike a tuple, every element of an array must have the same type
    // Arrays in Rust are different from arrays in some other languages 
    // because arrays in Rust have a fixed length, like tuples.
    // An array is a single chunk of memory allocated on the stack.
    // let a = [1, 2, 3, 4, 5];
    // let a: [i32; 5] = [1, 2, 3, 4, 5];
    // vector give us the flexibility of adding and removing elements

    another_function(5);

    // let x = 5;

    // let y = {
    //     let x = 3;
    //     x + 1
    // };
    
    // The block evaluates to a value that gets assigned to y
    // {
    //     let x = 3;
    //     x + 1
    // };

    let five = five();

    println!("the value of five is {} ", five);
}
// FUNCTIONS
fn another_function(x: i32) {
    println!("Value passed is {}", x);
}

fn five() -> i32 {
    5
}