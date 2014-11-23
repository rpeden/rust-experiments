fn main() {
    let x = 5i;
    
    let p = proc() { x * x };
    println!("calling proc. value is: {}", p());    
}
