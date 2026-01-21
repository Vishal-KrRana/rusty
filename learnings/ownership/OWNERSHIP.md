## Basic of ownership
  - the concept of ownership is evolved to prevent rust to get into unsafe program execution 
  - ownership is the discipline to be followed to ensure safety of rust programs.
  
lets learn it by example.
  
    fn read(y: bool) {
        if y {
            println!("y is true!");
        }
    }
    fn main() {
        read(x); // oh no! x isn't defined!
        let x = true;
    }

> In the above example the fn read trying to read before defination of x.

  - in many languages that won't be a problem but in rust rejects this idea because that can cause run time errors.
  - cause rust tries to complie efficient code and in above case to make it happen rust have to check it before initilizaion of the variable that's inefficient.
  - the other problem related to compiling that code is that there can be a value stored in that memory address (like\- 3, 0xff2397, etc ) and the read fn aspects a boolean value and if the value not found to be boolean the program crashes.

  > ### Memory model for rust programs:
  - lets understand stack and heap first 
  
  - stack: the stack is of fixied size and has very limited amount of memory like\- 1mb to 4mb depending on the os 
      - the stack is very fast and efficient as it's located inside of the cpu.
      - the size of stack is much lower than of the heap.
     
  - heap: the size of the heap much higher than the size of the stack as data is getting stored in the ram which provider much higher capacity then stack.
      - the os decides the size of the heap and it can be expanded when neede but same is not true for the stack.
      - speed offered by the heap is slower than stack but it offers higher capacity.

Now we have understood the basic of stack and heap now lets understand how rust works with the stack and heap.

    fn main() {
        let n = 5;
        let y = plus_one(n);
        println!("The value of y is: {y}");
    }
    fn plus_one(x: i32) -> i32 {
        x + 1
    }
  
> lets explore above program how memory is allocated for the execution.
  - firstly the entry point of program  main function is pushed onto the stack.
  - now the value 5 assigned to the variable n.
  - the the function plus_one is pushed onto the stack.
  - the variabel x assigned the valu of n and returns the value adding 1 to it.
  - when the value is returned from the function the function gets out of the scope and gets popped from the stack.
  - now the value returned from the function is stored in the variable y.
  - now the macro println prints the value on the terminal or sends it to stdout stream.
  - finally main function is popped from the stack and program ends.
  This is how the rust allocates and deallocates the stack memory it creates a frame for each scope and pushes it when the scope starts and pops it when the scope ends.
  > for more detailed and visual explanation check out: [rustbook](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html) or run [stack.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/ownership/stack.rs)
  
   - every variable created in the stack has its own memory space.
   - if one variable is assinged by another variable a copy of the value is stored in the variable not the original value. or reference to that variable memory address.

> ## Heap in rust: 

the box is used to allocate memory when needed into the heap and the boxes takes care of allocation and deallocation of the heap memory space it's not done manually like other programming languages like in C and Cpp with new, malloc for allocation of memory in heap and delete, free for deallocation of memory.
  
  - So how rust allocates and deallocates memory in the heap?
  - rust uses ownership and borrowing model for memory management of the heap.
  - when the varible owning and referencing the heap data gets out of the scope the heap memory is also freed by the box 
  - this stops referencing a value in heap after getting free which can cause program crash or uncertain behaviour.

lets understand it by example:

    fn main() {
      let a = Box::new([0; 1_000_000]);
      let b = a;
    }
  
  what happens in the above example would the box would try to free the heap memory twice?
  
  - here comes the borrowing model in play when the value of variable `a` has been assigned to the variable `b` then the ownership of the heap is transferred to the variable b and whenever the variable b gets out of the scope then the heap is freed.
  - there are a lot of collections which uses box model for heap memory allocation like\- String, Vec, HashMap.
  - the variable from whom ownership has been transferred cannot be used for referencing the heap data.

  > for more detailed and visual explanation check out: [rustbook](https://rust-book.cs.brown.edu/ch04-01-what-is-ownership.html) or understand by making change in [heap.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/ownership/heap.rs)


> ## Referencing and Borrowing
  - when u transfer the ownership to a function when the function scope ends the heap referenc also gets freed.
  - this is trouble some but we can work around it by returning the argumenst and storing them in new variables.
  - creation of new variable consumes memory so it's inefficient so there comes the referencing.
  - referencing allows programmer to give the access to the heap data without transferring the ownership but how?
      - instead of heap memory address the address of variable which is storing the memroy location of heap is passed to the function as argument or we can say that references is give as the argument.
      - like argument points to the variable who own the heap data and the variable points to the heap.
      - So ownership remains to the variable instead of arguments.
  - check [reference.rs](https://github.com/Vishal-KrRana/rusty/blob/main/learnings/ownership/reference.rs) for example code.

  ### Dereferencing
  rust does dereferencing for us automaticall but we can also do it manually by using `*`. the process of deferencing is important and we need to access the correct data and doing dereferencing wrong can cause some unexpected behaviour or errors.


  ### Simultaneous aliasing and mutation:
rust prevents us from simultaneous alisaing and mutation. why?
  - when the memory is allocated into heap the reference is assinged to a variable owning it.
  - when the resize of allocated memory happens the resize causes change of memory address and the reference to that varible changes.
  - let a variable points to a certain data in the heap memory and try to mutate it.
  - before the mutation let's say the resize of heap happens then the reference to that memory location is invalid. and mutation can cause some problems like crashing of program. 
  - this is why simultaneous aliasing and mutation should not happen in any program who is working with heap.
  - rust restrict this by Read(R), Write(W) and Ownership(O) access.
  - the R, W and O access is provided to variables referencing to heap data such that the simultaneous mutation or aliasing doesn't happen. which can cause program failures or crashes.
