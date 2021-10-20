fn main(){
  let fruit = vec!["Apple", "Pineapple", "Watermelon"];

  let buffer_overflow = fruit[4]; // tries to access at index 4, position 3
  assert_eq!(buffer_overflow, "banana");

}
// will throw error because I try to access to an index that doesn;t exist or is illegal in this vector.
