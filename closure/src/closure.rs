fn main() {
    let x = 5i;

    let printer = || { println!("x is: {}", x); };
    printer();

    let printer2 = |z: int| -> () { 
    	let y = z + 1i;
    	println!("y is: {} and x is: {}", y, x); 	
    };

    printer2(x);
}
