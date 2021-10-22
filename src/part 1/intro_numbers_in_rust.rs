use num::complex::Complex; // brings the Complex type into the local scope

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

fn numbers_type(){
  // Signed integers ranging from 8 to 64 bit.
  // i8, i16, i32, i64
  let temperture: i8 = - 32; // it's just an example, i32 is the type anno by default
  
  // Unsigned integers ranging from 8 to 64 bit.
  // u8, u16, u32, u64
  let result_sum_of_naturals: u8 = 2 + 8;
  
  // Floating-point numbers in 32-bit 64-bit variants.
  // f32, f64
  let student_notes: f32 = 5.8;

  // isize and usize:
  ///Integers that assume CPU's "native" width. For example, in 64-bit CPUs, usize and isize will be 64-bits wide.
  
  println!("Signed: {}, unsigned: {}, floating: {}", temperture, result_sum_of_naturals, student_notes);

}

fn comparing_number(){
 // Less than < 
  1.0 < 2.0;
 // Greater than >
  89 > 87;
 // Equal to = 
  1.0 == 1.0;
 // Unequal !=
 2.9 != 9.5;
 // Equal to or less than <= 
 1.0 <= 2.0;
 // Equal to or greater than >=
 3.0 >= 9.0;
}

// Something extra to remember is that we can't compare two different types.
// To appease the compiler we need to use the as operator to cast  one of the operants to other's type:

fn compare_two_numbers(a: i32, b: u16){
  // It is way safer to cast a type to a larger one (this is called promotion), 
  //that's why we casted u16 to u32, and it is less risky than casting to a smaller type.
  if a < (b as i32){ // we cast b to i32.
    println!("{} is less than {}", a, b);
  }
}

fn calc_with_complex_numbers(){
  let a = Complex { re: 2.1, im: -1.3 }; // Every Rust type has a literal syntax
  let b = Complex::new(11.1, 22.2); // Most types implement a new() static method.
  let result = a + b;
  println!("{} + {}", result.re, result.im); // Accesses1 fields with dot (.) operator
}
fn main(){
  //basic_operation_with_numbers();
  //basic_binary_oct_hex_base_numbers();
  //numbers_type();
  //comparing_number();
  //compare_two_numbers(8, 15);
  calc_with_complex_numbers();
}
