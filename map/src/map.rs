fn main() {
  let new_numbers = range(1i, 100i).map(|x| x + 1i).collect::<Vec<int>>();

  for num in new_numbers.iter() {
    println!("{}", num);
  }
  
  println!("\nCounting!");
  for i in std::iter::count(1i, 5i).take(5) {
  	println!("{}", i);
  }

}
