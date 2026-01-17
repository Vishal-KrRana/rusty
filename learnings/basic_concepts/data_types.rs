#![allow(unused)]
// every value in rust have certain data type which is used to reserve memory it can be: scalar, compuond
// Integer data type: rust provides several Integer data types based on the memory requirement
//      signed int: i8, i16, i32, i64, i128 and isize
//      unsiged int: u8, u16, u32, u64, u128 and usize
// floating data type: rust have two floating point representation type
//      default is f64 and another one is f32
// boolean data type: used for control flow can be either true or false
//      annotated using bool keyword.
// character data type: used to store character
//      in rust char is of size 4 byte which can hold ASII as well as other character like:- emojis, etc.
fn main() {
    // signed int
    let x: i8 = -54;
    let x2 = -254;
    // unsigned int
    let y: u8 = 200;
    let y2 = 303030;
    // floating
    let z: f32 = 3.00008;
    let z2 = 99.99999;
    // boolean
    let t = true;
    let f: bool = false;
    // character
    let a = 'a';
    let b: char = 'B';
    let special = '😻';
    println!("{special}");
    // Compound data types : array and tuple
    // tuple is general way of grouping together a variety of type.
    let tup: (i8, i32, u8, u32, f32, f64, bool, bool, char, char) =
        (x, x2, y, y2, z, z2, t, f, a, b);
    // tuples can also be destructured and can be made mutable using mut keyword.
    let (mut p, mut q) = (tup.0, tup.5);
    p += 1;
    q -= 2.0;
    println!("{:#?} {} {}", tup, p, q);
    // Arrays contains sequence of variables of same data type in consecutive memory location.
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];
    println!("{:#?}", months);
    let arr [i16; 5] = [1, 2, 3, 4, 7]; // i16 is the data type for array and 5 is the size of array
    let same = [64; 4]; // here 64 would be filled in all 4 array index.
}
