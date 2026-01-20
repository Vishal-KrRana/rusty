fn main() {
    println!("main pushed into the stack");
    let x = 0;
    stack_ex(x);
    println!("main popped from the stack");
}

fn stack_ex(mut x: i32) {
    println!("pushed into the stack: {x}");
    x += 1;
    if x == 5 {
        return;
    } else {
        stack_ex(x);
    }
    println!("popped form the stack: {x}");
}
