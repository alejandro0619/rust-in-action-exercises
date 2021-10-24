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
fn _add<T>(i: T, j: T) -> T { // The type variable T is introduce with angle brackets <T>. 
  i + j // This function takes two arguments of the same type annd return a value of that type.
}
fn main(){
  
}