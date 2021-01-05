# Brief Overview of Ownership

Ownership is Rust's most unique feature. It enables Rust to write programs
without needing to manually allocate memory (like C/++), while still
being able to run without a Garbage Collector (like JS/Python).

## Rules

- Each value in Rust has a variable: its __owner__
- There can only be one owner at a time
- When the owner goes out of scope, the value is dropped
  
This is checked at compile time, meaning that you have to be explicit
about when you want a value to be freed in memory.

```rs
fn main() {
 // the owner of the String is x
 let x = String::from("Hello");

 // we move the value inside this function.
 // now doSomething is the owner of x.
 // Rust will free the memory associated with x 
 // as soon as it goes out of "doSomething" scope.
 doSomething(x);

 // The compiler will throw an error since we tried to use the value x
 // but since we moved it inside "doSomething"
 // we cannot use it as we don't have ownership
 // and the value may have been dropped.
 println!("{}", x);
}
```

This is weird so please read it again.
