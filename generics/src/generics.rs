enum Result<T, E> {
  Ok(T), 
  Err(E),
}

fn inverse(x: f64) -> Result<f64, String> {
  if x == 0.0f64 { return Err("x cannot be zero!".to_string()); }

  Ok(1.0f64 / x)  
}

fn main() {
   
}
