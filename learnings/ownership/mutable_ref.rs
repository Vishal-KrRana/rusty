// mutable references lets us mutate the data inside the heap with reference of it maintaining the read write and own access for safety
fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3, 4]; // here the vector v stored into heap is mutable therefore the values can be changed.
    let v_ref: &mut i32 = &mut v[1]; // now v_ref references to the value at index one of vector v and has the read and own access. not the write access because v_ref is not mut and it's referencing not holding any original value.
    *v_ref += 2; // here *v_ref has the read and write access. it doesn't have own access as the ownership has been transfered to v_ref.

    println!("the mutated value: {}", *v_ref);
    // after this line as there would be no usage of v_ref and *v_ref the ownership is transferred back to original variable v.
    println!("the vector v: {:?}", v); // after this line the read write and ownership access would be removed from the v.
}
