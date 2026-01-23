fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3]; // here the v have the read write and own access.
    let num: &i32 = &v[2]; // here the num has read and own access as it's not mutable there no write acess

    println!("Third element is {}", *num); // here we are dereferencing the num and the derefereced *num has only read access

    // after the usage of num has ended the read write and own access returned to the original variable v as the ownership hasn't been transfered jut have given access
    v.push(4);
    // after above line the read, write and own access has been taken away from the v.
}
