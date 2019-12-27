// functions with arguments!

fn main() {
    print_args(add_one(40), 42);
}

fn print_args(x: i32, y: i32) {
    println!("arg 1 = {}, arg 2 = {}", x, y);
}

fn add_one(x: i32) -> i32 {
    x + 1
}

