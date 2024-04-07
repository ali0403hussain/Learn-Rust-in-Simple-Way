// -------------------------------------------------
// 			- Variables
// 			        - Definition
// 			        - Mutability
// 			        - Scope
// 			        - Shadowing
// 			 - Constants
// -------------------------------------------------

fn main() {
    // Definition
    let x = 10; // automatically gets type by value
    let x: i16 = 10; // explicit type i.e. 16 bit integer
    println!("x is {x}");

    // Mutability
    let mut y = 5;
    y = 10;

    //Immutable
    let s;
    s = 1 + 50;
    println!("s is {s}");

    // Scope
    {
        let z = 50;
    }
    //let s = z; // Comiler error

    // Shadowing
    let t = 10;
    let t = t + 10;
    println!("t is {t}");

    let u = 3;
    let u = 3.0;

    let v = 30;
    {
        let v = 40;
        println!("inner v is: {v}");
    }
    println!("v is: {v}");

    // Constants
    const MAX_VALUE: u32 = 100;
}
