#[derive(Debug)] // Allows the println!() macro to print the Cereal enum
enum Cereal {
  Barley, Millet, 
  Rice, Rye, 
  Spelt, Wheat
}

fn print_cereals(){
  let mut grains: Vec<Cereal> = vec![]; // initializes an empty vector of cereal
  grains.push(Cereal::Rye); // adds one item to the vector
  drop(grains); // delete grains and its content
  println!("The cereal is: {}", grains); // will throw an error because  try to access to a vector when it's deleted.
}

fn main(){
  print_cereals(); // will throw error.
}