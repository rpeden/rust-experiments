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
}
