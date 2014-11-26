use std::io::Timer;
use std::time::Duration;

fn main() {
  let (port, chan): (Port<int>, Chan<int>) = Chan::new();

  do spawn || {
    timer::sleep(Duration::seconds(5));
    println!("first computation");
    chan.send(5i);
  }

  println!("second computation");
  let result = port.recv();
}
