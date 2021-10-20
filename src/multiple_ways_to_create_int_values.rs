use std::rc::Rc;
use std::sync::{Arc, Mutex};

fn main(){
  let a = 10; // int on the stack
  let b = Box::new(20); //It on the heap, known as boxed integer
  let c = Rc::new(Mutex::new(30)); // Boxed int wrapped with a reference counter
  let d = Arc::new(Mutex::new(40)); //Int wrapped in an atomic reference counter and protected by a mutual exclusion lock
  println!("a:{}, b:{}, c:{}, d:{}", a, b, c, d);

}