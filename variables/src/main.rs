// Basics of variables in Rust

fn main() {
    let x: u32 = 32;    
    println!("The value of x is {}", x);
    // variables are immutable
    // x = 16 // ILLEGAL
    let mut y = 16;
    println!("The value of y is {}", y);
    y = 10;
    println!("The new value of y is {}", y);
    const MAX_VALUE: u32 = 100_000;
    println!("Constant vaue MAX_VALUE = {}", MAX_VALUE);
}

