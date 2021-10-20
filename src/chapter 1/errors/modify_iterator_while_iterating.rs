fn main(){
  let mut letters = vec!["a", "b", "c"];

  // iterate over letters:
  for letter in letters {
    println!("{}", letters);
    letters.push(letter.clone()); // copies each letter and appends it to the end of letters.
  }
}

// This will throw an error due I can't modify an iterator while interating on it.
