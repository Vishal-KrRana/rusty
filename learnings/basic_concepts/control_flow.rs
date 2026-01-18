// The control flow is the basic building block in programming it executes a block of code when the condition is met
// the if else block and else if can be use to control the flow of program it run a block of code only if the condition is met meaning its evaluated to true.
// loops repeatedly does a work until a condition is fullfilled there are three types of loops in rust: loop, while and for
fn main() {
    let mut con = true;
    if con {
        println!("condition is true");
    }
    // if can be combined with else when condition inside if evalutes to false then else block would be executed.
    if !con {
        println!("condition is false");
    } else {
        println!("inside else block");
    }
    // if can be combined with else if too when if is false then the control goes to consecutive else if and so on.
    if !con {
        println!("in first if block");
    } else if con {
        println!("changed the condition from {con} to {}", !con);
        con = !con;
    } else if !con {
        println!("in third block");
    } else {
        println!("here i am in else block");
    }
    // loop is and infinite loop untill it's broken manually by pressing ctrl + c or by programming using break keyword it will loop forever.
    let mut count = 0;
    // while loop is used when the codition inside loops stops it.
    while count != 10 {
        count += 1;
        println!("{count}");
    }
    // for loops is used to iterate over sequentila structures like array.
    let arr = [5, 4, 3, 2, 1];
    for a in arr {
        println!("{a}");
    }
    'b: loop {
        println!("press ctrl+c to stop me");
        //loop can be combned with label to break it
        'a: loop {
            count += 1;
            println!("{count}");
            if count == 10 {
                break 'b;
            } else {
                break 'a;
            }
        }
    }
}
