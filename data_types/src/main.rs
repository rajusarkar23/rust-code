fn main() {
    println!("Hello, world!");
    // RUST NUMERIC OPERATIONS
    // addition
    let sum = 5 + 10;

    // subtraction
    let difference = 95.5 - 4.3;

    // multiplication
    let product = 4 * 30;

    // division
    let quotient = 56.7 / 32.2;
    let truncated = -5 / 3; // Results in -1

    // remainder
    let remainder = 43 % 5;
    // booleans
    let t = true;

    let f: bool = false; // with explicit type annotation

    // char
    let c = 'z';
    let z: char = 'ℤ'; // with explicit type annotation
    let heart_eyed_cat = '😻';


    // tuple
     let tup: (i32, f64, u8)   = (500, 6.4, 1);
}

// rust data type subsets => scalar and compound.

// Scalar =>
// rust has four scalar types: integers, floating-point-number, Booleans and characters.
// integers =>
// an integers is a number without fractional component.
// integer types => unsigned => u, signed => i.
// i8/u8 .... i128/u128, isize(arch)/usize(arch) {arch => computer architechture}.
// each signed variants can store numbers from -(2^n-1) to 2^n-1 -1, where n is the number of bits,
// so an i8 can store number from -(2^7) to 2^7 - 1 => from -128 to 127.
// and an unsigned can store from 0 to (2^n - 1) => 0 to 255
// floating-points =>
// rust's floating-point types are => f32 and f64
// default type is f64,
// bool =>
// Booleans are one byte in size.
// char =>
// Rust’s char type is the language’s most primitive alphabetic type.
// in rust we write char using single quote 'char'.

// compound => rust has two primitive compund types,
// tuples and arrays.
// We create a tuple by writing a comma-separated list of values inside parentheses.