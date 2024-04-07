// -------------------------------------------------
// 			- Scalar Data Types
//	 			- Integers
// 				- Floats
// 				- Chars
// 				- Boolean
// -------------------------------------------------
fn main() {
    // Unsigned integers
    let unsigned_num: u8 = 5;

    // Signed integers
    let signed_num: i8 = 5;

    // Floating point numbers
    let float_num: f32 = 5.0;

    // Platform specific integers
    let arch_1: usize = 5;
    let arch_2: isize = 5;

    // Characters
    let char = 'a';
    println!("Value of char is {char}");

    // Boolean
    let b: bool = true;

    // Type aliasing
    type Age = u8;
    let peter_age: Age = 42;
    println!("Peter age is {peter_age}");

    // Type Conversion
    let a: u32 = 10;
    let b: u8 = a as u8;
    println!("a is {a} and b is {b}");
}
