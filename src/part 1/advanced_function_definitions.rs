use std::ops::{Add}; // brings Add to local scope
use std::time::{Duration};

// Explicit lifetime annotations:
// As a bit of forewarning, allow me to introduce some more complicated notation. As you read through Rust code, you might encounter definitions that are hard to decipher because those look like a hieroglyphs from an ancient  civilization.
// <'a, 'b> declares two lifetimes variables. 'a and 'b, within the scope of add_with_lifetimes.
// These are normally spoken as lifetime a and lifetime b.

// i: &'a i32 binds lifetime variable 'a to the lifetime of i, the syntax reads as "parameter i is a reference to an i32 with lifetime a" the same with j

fn _add_with_lifetimes <'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j
}


// Generic Functions:

// Another special case of function syntax appears when programmers write Rust functions to handle many possible input types.
// So far, we have seen functions that accept 32-bits integer (i32). The following code can be called by many input types as long as these are the same.
// The type variable T is introduce with angle brackets <T>. 
// This function takes two arguments of the same type annd return a value of that type.
// But this example below WON'T compile, because <T> stands for any type, include those types where adding is not supported.
// To add support to this T, we need to introduce new terminology.
// All of Rust's operators, including addition, are defined within traits. 
//To require that type T must support adddition, we include a type bound alongside the type variable in the funtion's definition.
// The fragment: <T: std::ops::Add<Output = T >> says that T must implement std::ops::Add. 
//Using a single type variable with the trait bound ensures that arguments as i and j, as well the result type, are the same type and that their supports addition.

// But what's a trait?
// Is a language feature that is analogous to an interface, protocol or contract. If you have a background in
// OOP, consider a trait to be an abstract base class. If you have a background in Functional programming, 
// Rust's traits are close to Haskell's type clases.
fn _add<T: std::ops::Add<Output = T >>(i: T, j: T) -> T { 
  i + j 
}


// Generic function with a type variale and trait bounds.
fn _add_version_two<T: Add<Output = T>>(i: T, j: T) -> T {
  i + j 
}

fn main(){
 
}
