// a reference is a value  that stands in place for another value. 
//For example, imagine that variable A is a large array is that is costly to duplicate. In some sense, a reference R is a cheap copy of A. 
//But instead of creating a duplicate, the program stores A's adress in memory. 
//When the data of A is required, R can be dereferenced to make A available. The following code shows it better:

fn _references(){
  let a = 42; 
  let r = &a; // R references to A
  let b = a + *r; // Adds A to A (via dereferencing R) and bind the result to B

  println!("A + A = {}", b); // shows 84ls
}
// References are created with the reference operator (&) and the dereferencing occurs with the dereference operator (*)

fn example() {
  let needle = 0o204;
  let haystack = [1, 1, 2, 5, 15, 52, 203, 877, 4140, 21147, 132];

  for i in &haystack { // iterates over references to elements within haystack
    if *i == needle { // *i returns the the item's referent
      println!("{}", i);
    }
  }
}
fn main(){
  example();
}