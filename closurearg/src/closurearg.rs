fn main() {
    let square = |x: int| { x * x };

    let result = twice(5i, square);
    println!("result is: {}", result);
}

fn twice(x: int, f: |int| -> int) -> int {
    f(x) + f(x)
}
