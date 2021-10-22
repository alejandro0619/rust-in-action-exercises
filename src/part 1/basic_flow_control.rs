// For: the central pillar of iteration
fn for_loop() { 
  let container = vec!['a', 'b', 'c'];

  for item in container {
    println!("{}", item);
  }
  // after iterating throught container, we cannot access to it, because its lifetime has ended.
  // Even tho' container remains in the same local scope.

  // To avoid that, we should pass container as a reference:

  let container_2 = vec!['a', 'b', 'c', 'd'];
  for item in &container_2 { 
    println!("{}", item);
  }
  println!("container 2 first item: {}", container_2[0]);
}

fn main(){
  //for_loop();
}