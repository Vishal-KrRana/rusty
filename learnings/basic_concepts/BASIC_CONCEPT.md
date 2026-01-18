> ## This folder contains: Common Programming Concepts
the common programming concepts are those concepts which are common for all languages like variables, data types, functions, comments and control flow these concepts are found in every language with their own way of implementing it. The way of implementations of these can be similar in multiple languages as these concepts are common and adapted from one to other.

> ### Variables and Mutability in rust:
  - by default variables are immutable in rust.
  - to make a variable mutable we need to use keyword `mut` after `let`
  - without `mut` the variables throws an error on reassignment.
  - the data type can be annotated at the time of variable creation or it can be decided by compiler at the time of compilation by the value assigned to it.
  
> example can be found in [var_mut.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/var_mut.rs "click here to view var_mut.rs")

> ### Constants in rust:
  - constants are also immutable in rust which are defined using keyword: `const`.
  - it can't be mutated unsing `mut` keyword.
  - constast must be annotated \( defined the type of data type it will hold\) at the time of creation. 
  - rust uses it to assign constant terms like: speed of light, value of pi, etc.
  - The constants name are defined using capital snake case. ex: `const PI: f32 = 22/7;`.
  - In the above example the value of PI is calculated at complie time not at the run time which is only true for `const` not for other variable like `let` for let the compiler may or may not evaluate the expression.
  - constanst are valid within the scope of the declaration of the constant and entire time a program runs.
> example of constants can be found in [constants.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/constants.rs "click here to visit constants.rs")

> ### Shadowing in rust:
  - Shadowing is the process of assigning new value to a variable without creating new variable name \(confusing?\).
  - it is same as assignement but without the drawbacks of assignment as the new value assign to variable must be of same data type else and error would be thrown but with shadowing we can assign different data type to new varibale with same name without any error.
  - it's mostly used in type casting \(type conversion\).
  - both variable have same name but are not same variable \( doesn't share same memory space.\)
  - the new variable overshadows older one untill the scope ends.
  
> example of shadowing can be found in [shadowing.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/constants.rs "click here to visit shadowing.rs")

> ### Data Types in rust:
####  Rust supports scalar \(integer, floating, boolean, character\) and compound \(tuple, array\) data types

## Scalar Types: Integer, Floating, Boolean, Character
### Integer: this is an which holds the integer numbers. it can be further divided in signed and unsigned integers
  > signed int : these can be annotated with variable name by i<size> like\- `let var: i16 = 2222;`
  1. `i8` : can hold value from \-128 to \+127
  2. `i16`: can hold value from \-2^15 to \(2^15 \-1\)
  3. `i32`: can hold value from \-2^31 to \(2^31 \-1\)
  4. `i64`: can hold value from \-2^63 to \(2^63 \-1\)
  5. `i128`: can hold value from \-2^127 to \(2^127 \-1\)
  6. `isize`: this is architecture specific
  
  > unsigned int : these can be annotated with variable name by u<size> like\- `let var: u8 = 200;`.
  - it can hold value from 0 to 2^size
  - ex: u8, u16, u32, u64, u128, usize

> 1. here size refer to number of bits of memory allocated for storing the variable
> 2. for architecture specifi like: isize, usize the varies like for 32bit architecture it would be 32 and for 64bit architecture it would be 64.
> 3. The defalut types for compiler is i32 until you annotate the varible with specific type the compiler will interpret the value as i32 for interger types.
    
### Floating: these are used for floating point representation like: 3.14, etc
  1. It has two types like: `f32` and `f64`.
  2. The default is `f64` as in mordern computer f32 and f64 take almost same time and f64 is more precise.

### Boolean: used to store true of false used in flow control.
  - can be annotated using `bool` keyword. ex: `let t: bool = true;`.
  - only have two possible values `true` or `false`.

### Character: used to store character like: ASCII, UTF\-8, UTF-16, etc.
  - can be annotated using `char` keyword. ex: `let a: char ='a';`
  - size of char in rust is 4 byte so it can store character of multiple languages like: japanese, chinese, korean, etc.

## Compound Types: Arrays and Tuples
compound can group multipe values of different or same data types into a single variable.
  1. Tuple: tuple can group multiple types into a single variable
  - ex: `let tup: (i8, char, bool) = (12, 'A', false);` or `let tup = (55, 'B', true);`
  - these values can be destructured to use in the program.
  - tuple values can be accessed using its index like: `tup.0`.
  
  2. Array: array is a linear data structure which holds the consecutive values of same type.
  - ex: `let a = [1, 2, 3, 4];`
    - `let a: [i16; 5] = [1,2,3,7,9];` : here a array would be created of size 5 and `i16` type.
    - `let a = [6; 5];` : array of size 5 would be created with all values filled by 6.
  - we can access the elements of array by their index like: `a[0]` this would hold the value at index zero.

> example of all the data types can be found in [data_types.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/data_types.rs "click here to visit data_types.rs")

> ## functions in rust
  1. before starting functions we need to know about expression and statement in rust
      - expressinons returns value after evaluation like `let x = 6 + 1;` 
      - In above example the value 6 + 1 is an expression and returns a value 7 which would be stored in the variable x.
      - The whole term `let x = 6 + 1;` is a statement which doesn't returns a value like expressin therefore can't be used like `let x = y = 6 + 1;`
          - in above y is a unassigned variable which would be assigned by 7 but the x wouldn't be assigned by 7 it would be empty as `y = 6 + 1;` is a statement which doesn't returns a value for x to store.
          - try to run [statement_expr.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/statement_expr.rs "click here to visit statemen_expr.rs") and see the result.
      - the expressions can be used with functions to return a value directly without using return keyword.
  2. Types of function in rust: rust have function with parameter, without parameter, returning value, doesn't returns value.
      - check the example in [functions.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/functions.rs "click here to visit functions.rs")

> ## Comments in rust
  - the comments in rust serves the purpose of improving code readibility for developers and for automatically creating the doc for the code base on the basis of comments.
  - Types of comments in rust:
      1. `//` : this is single line comment which comments out entire line after its apperance the statements after it wouldn't read by the rust compiler.
      2. /*<br>
      explanations in multiple lines<br> 
      line 2 <br>
      line 3<br>
      ...<br>
      */ comment ends here everything inside it would be ignored by complier.

> ## Control Flow in rust
The control flow decides how the program would evaluated and how to proceed of the basis of conditions.
  1. Branching statements: if, else, else if.
      - if block is executed if the condition is met or condition evaluated to tru
      - else block is run if the condition above the else block in `if` and `else if` is not met the desired condition. There is no condition is required to get into else block like `if` and `else if` blocks
      - else if block is exeuted when the above blocks of `if` and `else if` fails to met their conditions.
  2. Loops statements: loop, while, for
      - loop: it is and infinite loop which runs infinitely undill manual intervantion like pressing ctrl+c or breaking the loop by `break;` when a certain condition is met. It doesn't require a condition to get inside the loop it's condition less loop.
          - loop can be labeled to break the loop
          - the lable follows the pattern `'label_name: loop { loop body }`
          - when we need to break the loop we can do it using break with lable like `break label_name;`.
      - while: the while loop is a loop with condition which stops when a certain condition is met inside the while loop block.It converts the condition which runs the while loop to false and the while loop breaks.
      - for: the for loop is used to iterate over a continious data structure like array without going out of bounds. which makes it safe to iterate over the array without going out of the bounds. 
      
> check out [control_flow.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/basic_concepts/control_flow.rs "click here to visit control_flow.rs") to see examples of control flow
