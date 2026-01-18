// functions are prevalent in rust like all other languages
// the entry point for all programs is the main function in rust

// a simple function example without parameters would be:
fn namaste() {
    println!("namaste");
}
// function with parameters
fn print_arg(x: i32, c: char) {
    println!("the argument was: {x}:{c}");
}
// the above functions was only with statement the example with expression is:
fn double(x: i32) -> i32 {
    // -> i32 defines that the function would return value of i32 type.
    x * 2 // here it would not end with ";" because it's an expression and the value evaluted by expression would be returned
}

fn main() {
    namaste();
    print_arg(2, 'h');
    println!("the double of 2 is: {}", double(2));
}

// statement doesn't returns a value where expression does returns a value on evalution of expression therefore statement can't be used to return value.
