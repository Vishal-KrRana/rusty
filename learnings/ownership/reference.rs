fn main() {
    let s1: String = String::from("namaste");
    let s2: String = String::from("world");
    let r1: String = String::from("hello");
    let r2: String = String::from("duniya");
    // below we are not using reference so we need to return the values from the function and store it into new variable
    let (s1, s2) = greet_own(s1, s2); // here we are using shadowing to store values with same variable name.
    greet_ref(&r1, &r2); // here we are usning reference so we don't need to return the values and store it into new variable the r1 and r2 still owns the string.

    println!("{} {}", s1, s2); // try different name for the new variables to store return value and the try to access s1 and s2
    println!("{} {}", r1, r2);
}

fn greet_own(x: String, y: String) -> (String, String) {
    println!("i own the value: {x} and {y}");
    (x, y) // this tuple would return value that can be stored in new variable. or under same name by shadowing
}

fn greet_ref(x: &String, y: &String) {
    println!("I don't own the values: {x} and {y} i just refer to them");
    // here we don't need to return the values as the x and y doesn't owns them they just refer to them.
}
