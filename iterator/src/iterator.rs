fn main() {
   for x in range(0i, 10i) {
       println!("{}", x);
   }
   
   let nums = vec![11i, 12i, 13i];
   for num in nums.iter() {
      println!("{}", num);
   }

   let greater_than_55 = range(0i, 125i).find(|x| *x > 55);
   
   match greater_than_55 {
     Some(_) => println!("Houston, we have numbers"),
     None    =>	println!("Nothing found :("),
   }

   let sum = range(1i, 4i).fold(0i, |sum, x| sum + x);
   println!("sum is: {}", sum);

   let some_numbers = range(1i, 1000i)
                      .filter(|&x| x % 2 == 0)
                      .filter(|&x| x % 3 == 0)
                      .take(5)
                      .collect::<Vec<int>>();

    for num in some_numbers.iter() {
      println!("{}", num);
    }

}
