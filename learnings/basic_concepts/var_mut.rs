// variable and mutability
fn main() {
    let x = 5; // this variable is immutable by default to make it mutable we need to use mut keyword after let
    let mut y = 10; // new values can be assigned in the variable y
    y = x * y;
    // x = y / x; // this line would give us and error as we are assigning new value to a immutable variable
    println!("value of x: {x} and y is: {y}");
}
