fn main() {
   println!("Type something:");

   let input = std::io::stdin().read_line().ok().expect("Failed to read line");

   println!("{}", input); 
}
