fn inverse(x: f64) -> Result<f64, String> {
  if x == 0.0f64 { return Err("x cannot be zero!".to_string()); }

  Ok(1.0f64 / x)  
}

fn main() {
   let x = inverse(25.0f64);

   match x {
    Ok(x) => println!("The inverse is: {}", x),
    Err(msg) => println!("Error: {}", msg),
   }
}
