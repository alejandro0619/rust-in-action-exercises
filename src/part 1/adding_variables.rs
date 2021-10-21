fn main(){
  let a = 10; // The compiler can infer the type of a variable depending on the value.
  let b: i32 = 20; // you can declare the type of a value
  let c = 30i32; // numerical type can also include a type annotation in their literal
  let d = 39_i32; // The numbers can also contain underscore for readability and have no functional impact.

  let result = add(add(a, b), add(c, d));
  println!("The result is of the sum is: {} ", result);
}

fn add(first_number: i32, second_number: i32) -> i32 { //Type declaration are required when definig a function
  first_number + second_number // the function will return the last expression so that return statement is not required.
}