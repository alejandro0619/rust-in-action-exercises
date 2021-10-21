// This section digs into how to use numbers in rust and operate with them.

// Int and decimal (floating-point)
// Operations on numbers uses infix notation like the most programming languages out there
// Some notable differences are:
// Rust includes a large number of numeric types.
// Conversion between types are always explicit.
// Rust's numbers can have method, for example, to round 24.5 to the nearest int, you can use: 24.5_f32.round() rather tha round(24.5_f32) here the type suffix is required because a concrete type is required



fn basic_operation_with_numbers(){
  let twenty = 20; // infers type
  let twenty_one: i32 = 21; // explicit type annotation
  let twenty_two = 22_i32; // type annotation in the literal
  let addition = twenty + twenty_one + twenty_two;
  println!(
    "Twenty ({}) plus twenty one ({}) plus twenty_two ({}) is equal to: {}",
    twenty, twenty_one, twenty_two, addition
  );
  
  let one_million: i64 = 1_000_000;
  println!("The 2nd power of {}, is {}", one_million, one_million.pow(2));

  // Creates an array of numbers, which must all be the same type, by sorrounding those with square brackets.
  let forty_twos = [
    42.0,
    42f32,
    42.0_f32
  ];

  println!("{:02}", forty_twos[0]); // Elements within the array can be accessed numerically, starting at zero.
}


// Base 2, 8, and base 16 numeric literals:

fn basic_binary_oct_hex_base_numbers(){
  let three = 0b11; // 0b prefix indicates binary (base 2) numeral.
  let thirty = 0o36; // The 0o prefix indicates octal (base 8) numerals
  let three_hundred = 0x12c; // The 0x indicates hexadecimal (base 16) numerals

  println!("base 10: {} {} {}", three, thirty, three_hundred);
  println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
  println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
  println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);

}
fn main(){
  //basic_operation_with_numbers();
  basic_binary_oct_hex_base_numbers();

}
