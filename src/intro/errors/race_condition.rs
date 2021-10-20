use std::thread; // brings multi threading into local scope

fn main(){
  let mut data = 100;
  thread::spawn(| | { data = 500; }); // spawn takes a closure as an arguments
  thread::spawn(| | { data = 1000; });
  println!("{}", data);
}

// will throw an error because we don't know what value data will take when the applicatio is executing.
