use std::fs::File;
use std::io::{self, prelude::*, BufReader};

fn main() -> io::Result<()> {
    let file = File::open("input_01.txt")?;
    let reader = BufReader::new(file);

    let list_of_int:Vec<i32> = reader
    .lines()
    .into_iter()
    .map(|x|  x.unwrap().parse::<i32>().unwrap()).collect();
    
    let nb_01 = list_of_int.windows(2).filter(|x| x[0]< x[1]).count();
    println!("{}",nb_01);

    let nb_02 = list_of_int.windows(3).map(|x| x.iter().sum()).collect::<Vec<i32>>().windows(2).filter(|x| x[0]< x[1]).count();
    println!("{}",nb_02);
   
  

   let r = BufReader::new(File::open("input_01.txt")?)   
   .lines()
   .into_iter()
   .map(|x|  x.unwrap().parse::<i32>().unwrap())
   .collect::<Vec<i32>>()
   .windows(3)
   .map(|x| x.iter().sum())
   .collect::<Vec<i32>>()
   .windows(2).filter(|x| x[0]< x[1])
   .count();
   println!("{}",r);
   Ok(())


}
