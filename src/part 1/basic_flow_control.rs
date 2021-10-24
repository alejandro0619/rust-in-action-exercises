use std::time::{ Duration, Instant }; // for the while_loop example

// For: the central pillar of iteration
fn _for_loop() { 
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

fn _anon_loops() {
   // When a local variable is not used within a block, by convention, you'll use an underscore (_). Using this pattern in conjunction with the _exclusive range syntax_ (n..m)
   // and the _inclusive range syntax_ (n..=m) makes it clear that the intent is to perform a loop for a fixed number of times. Here's an example.__rust_force_expr!
   
   for _ in 0..10 {
     // code in here:
     println!("The loop is going over and over");
   }
}

fn _basic_for_loop_with_index() {
  // It's quite common to loop through things by using a temporary variable that's incremented at the end of each iteration.
   
  let collec = [1, 2, 3, 4, 5];
  for i in 0..collec.len() {
    let item = collec[i];
    println!("Index: {}, value: {}", i, item);
  }
}

fn _using_continue() {
  for n in 0..10 {
    if n % 2 == 0 {
      println!("{} This number is even", n);

    } else {
      println!("{} Is not even", n);
    }
  }
}

fn _incr_count_using_while() {
  // This is not meant to be a real benchmark but it is useful to show how does the while loop works.
  let mut count = 0;
  let time_limit = Duration::new(1,0); // creates a duration that represents 1 second
  let start = Instant::now(); // Accesses time from the system's clock.
  println!("{:?}", Instant::now() - start ); // wtff does this means
  
  while (Instant::now() - start) < time_limit { // An Instant minus an Instant return a Duratiion
    count += 1;
  }
  println!("{}", count);
}
fn _using_loop() {
  // This will be executing forever until a break statement is reached.
  // loop {}
  // and it's often seen when implementing long-running-servers.
}

fn _break_nested_loop() {
  // You can break out of a loop using loop labels. A loop label is an identifier prefixed with an apostrophe ('), like this example shows:
  'outer: for x in 0.. {
    for y in 0.. {
      for z in 0.. {
        if x + y + z > 1000 {
          break 'outer;

        }      
        // ...
      }
    }
  } 
}

fn _conditional_branching() {
  let item = 65;
  if item == 54 {
    // ..
  } else if item == 78 {

  } else {
    //..
  }
}

fn _match_pattern() {
  // While it's possible to use if/else blocks, match provides a safer alternative.
  // Match warns you if you haven't considered a relevant alternative. It is also elegant and concise:
  let item = 78;

  match item {
    0 => {}, // to match a single value, provide the value. No operator is required
    10..=20 => {}, // The ..= syntax matches an inclusive range
    40 | 80 => {}, // The vertical bar (|) matches values on either each side of it
    _ => {}, // The underscore matches every value
  }
}

fn _match_mutiples_values() {
  let needle = 42;
  let haystack = [1, 1, 2, 5, 14, 42, 132, 429, 1430, 4862];

  for i in haystack {
    let result = match i {
      42 | 132 => "Hit!!",
      _ => "Miss",
    };

    if result == "Hit!!" {
      println!("{}: {}", i , result);
    } else {
      println!("Didn't hit")
    }
  }
}


fn main(){
  
}