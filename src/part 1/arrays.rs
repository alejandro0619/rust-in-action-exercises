// List of things are incredibly common. The two types that you will work with most often are arrays and vectors
// Arrays are fixed-width and extremely lightweight.
// Arrays as far as Rust concerned, is a tighly-packed collection of the same thing. 
//It's possible to replace items within an array, but it's size cannot change once it's set. 
//Because variable-length types like String add a degree of complication, we'll revert back to discussing numbers for a while.


fn main(){
// We can create arrays by just typing: [el1, el2, el3, ...]
// types 
let _array_of_friends:[&str; 3] = ["John", "Daniel", "Alex"];

// Or you can just repeat the element on the left many times as it's indicated on the right: 
let array_of_numbers_two = [2;9];

for e in &array_of_numbers_two {
  println!("{:?}", e);
}
}