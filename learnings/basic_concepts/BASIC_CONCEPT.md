> ## This folder contains: Common Programming Concepts
the common programming concepts are those concepts which are common for all languages like variables, data types, functions, comments and control flow these concepts are found in every language with their own way of implementing it. The way of implementations of these can be similar in multiple languages as these concepts are common and adapted from one to other.

> ### Variables and Mutability in rust:
  - by default variables are immutable in rust.
  - to make a variable mutable we need to use keyword `mut` after `let`
  - without `mut` the variables throws an error on reassignment.
  - the data type can be annotated at the time of variable creation or it can be decided by compiler at the time of compilation by the value assigned to it.
  
> ### Constants in rust:
  - constants are also immutable in rust which are defined using keyword: `const`.
  - it can't be mutated unsing `mut` keyword.
  - constast must be annotated \( defined the type of data type it will hold\) at the time of creation. 
  - rust uses it to assign constant terms like: speed of light, value of pi, etc.
  - The constants name are defined using capital snake case. ex: `const PI: f32 = 22/7;`.
  - In the above example the value of PI is calculated at complie time not at the run time which is only true for `const` not for other variable like `let` for let the compiler may or may not evaluate the expression.
  - constanst are valid within the scope of the declaration of the constant and entire time a program runs.

> ### Shadowing in rust:
  - Shadowing is the process of assigning new value to a variable without creating new variable name \(confusing?\).
  - it is same as assignement but without the drawbacks of assignment as the new value assign to variable must be of same data type else and error would be thrown but with shadowing we can assign different data type to new varibale with same name without any error.
  - it's mostly used in type casting \(type conversion\).
  - both variable have same name but are not same variable \( doesn't share same memory space.\)
  - the new variable overshadows older one untill the scope ends.
> ### Data Types in rust:
####  Rust supports scalar \(integer, floating, boolean, character\) and compound \(tuple, array\) data types
> Scalar Types: Integer, Floating, Boolean, Character
  #### - Integer: this is an which holds the integer numbers. it can be further divided in signed and unsigned integers
  > - signed int : these can be annotated with variable name by i<size> like\- `let var: i16 = 2222;`
  1. `i8` : can hold value from \-128 to \+127
  2. `i16`: can hold value from \-2^15 to \(2^15 \-1\)
  3. `i32`: can hold value from \-2^31 to \(2^31 \-1\)
  4. `i64`: can hold value from \-2^63 to \(2^63 \-1\)
  5. `i128`: can hold value from \-2^127 to \(2^127 \-1\)
  6. `isize`: this is architecture specific
  > - unsigned int : these can be annotated with variable name by u<size> like\- `let var: u8 = 200;`
    - it can hold value from 0 to 2^size
    - ex: u8, u16, u32, u64, u128, usize
  
  > - here size refer to number of bits of memory allocated for storing the variable
  
  > - for architecture specifi like: isize, usize the varies like for 32bit architecture it would be 32 and for 64bit architecture it would be 64.
  
  > - The defalut types for compiler is i32 until you annotate the varible with specific type the compiler will interpret the value as i32 for interger types.
  #### - Floating: these are used for floating point representation like: 3.14, etc
    1. It has two types like: `f32` and `f64`.
    2. The default is `f64` as in mordern computer f32 and f64 take almost same time and f64 is more precise.
