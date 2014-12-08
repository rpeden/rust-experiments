struct Circle {
  x: f64,
  y: f64,
  radius: f64
}

impl Circle {
  fn area(&self) -> f64 {
    std::f64::consts::PI * (self.radius * self.radius)
  }

  fn new(x: f64, y: f64, radius: f64) -> Circle {
  	Circle {
  		x: x,
  		y: y,
  		radius: radius
  	}
  }
}

fn main() {
  let c = Circle { x: 0.0, y: 0.0, radius: 2.0 };
  println!("Circle's area is {}", c.area());

  let newCircle = Circle::new(1.0, 1.0, 34.0);
  println!("newCircle's area is {}", newCircle.area());
}
