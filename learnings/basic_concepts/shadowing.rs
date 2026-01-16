// Shadowing is rust
fn main() {
    let x = 10;
    println!("value of x is: {x}");
    // x = 20; // this is not allowed as x is immutable
    // but we can shadow the variable x by shadowing it
    let x = 20; // now previous value of x is shadowed by the new value the complier will read new value instead of the previous value of x
    println!("new value of x is: {x}");
    {
        let mut x = x;
        x = x * 2;
        println!("value of x within this new scope: {x}");
        let x = 20 + x;
        println!("value of shadowed x within new scope: {x}");
    }
    println!("value after scope ended: {x}");
}
