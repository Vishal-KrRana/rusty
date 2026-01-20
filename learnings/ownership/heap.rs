fn main() {
    let s: String = String::from("Namaste");
    // the below variable s would be the owner of the new string allocated in the heap and heap would be freed when it gets out of the scope.
    let s: String = add_duniya(s); // here we are using shadowing to store the returned string under the same name we can used another varible name too
    println!("the result: {}", s); // if we haven't shodowed the variable with the referencing s would cause error. then we needed to reference the new variable name.
}

fn add_duniya(mut s: String) -> String {
    s.push_str(" Duniya");
    s // if wouldn't have return string s then we would caught an error as the scope of the variable s would end here therefore the heap would be freed too.
}
