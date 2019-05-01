

fn main() {

    // rust is **statically typed**, and needs to know the types of all variables
    // at compilation time.

    // SCALAR TYPES represent a single value

    let my_scalar: char = 'a'; // a scalar type

    // 4 types in total.
    
    // 1) Interger Types
    // base2 (8-128), signed (iXX) and unsigned (uXX)

    let days_till_2018: i8 = -100;
    let age: u8 = 30;
    let housing_prices: u16 = 300_000; // 300,000

    // unsigned numbers cannot be negative
    // let bad: u32 = -1; // !! error[E0600]: cannot apply unary operator `-` to type `u32`

    // other ways to write ints
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A'; // u8 only



}

fn number_operations() {

    let sum = 5 + 10;

    let difference = 95.3 - 4.3;

    let quotient = 67.7 / 32.2;

    let remainder = 43 % 3;

    // standard!
    
}

/// boolean types ///

fn booleans() {
    let t = true;
    let f: bool = false;
    // the usual
}

/// Character Types

fn chars() {
    let c = 'z'; // use single quotes for chars (different than strings!)
    let z = 'z';
    let heart_eyed_cat = '😻'; // unicode by default. Meaning stuff like emojis work!
}

/// COMPOUND TYPES ///
/// this is where things get interesting...
///
// like in most languages, compound types become a mix of many types of scalar types.

/// tuple type

fn tuple() {
    let tup (i32, f64, u8) = (500, 6.4, 1); // holds a fixed amount of any types (each one declared)

    // using pattern matching you can destructure

    let (x, y, z) = tup; // assigns each value to x, y and z in order
    
    let my_birthday (u32, u32, u32) = (1989, 1, 28)
}

