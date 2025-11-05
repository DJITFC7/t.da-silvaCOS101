use std::io;

fn main() {
   
   println!("Enter customer name");
   let mut name = String::new();
   io::stdin().read_line(&mut name).expect("Not a valid input");
  

   println!("Enter units consumed");
   let mut unit_input = String::new();
   io::stdin().read_line(&mut unit_input).expect("not a valid input");
   let unit:f32 = unit_input.trim().parse().expect("not a valid number");

 

  if unit >= 0.0 && unit <= 100.0 {
    let total_cost = unit * 20.0;
    println!("total cost is {}", total_cost );
  }else if unit > 100.0 && unit <= 300.0 {
      let total_cost = unit * 35.0;
      println!("total cost is {}", total_cost );
  }else if unit > 300.0 {
    let total_cost = unit * 50.0;
    println!("total cost is {}", total_cost );
  }else if unit > 500.0 {
    let total_cost = (unit * 50.0) + 5_000.0;
    println!("total cost is {}", total_cost );
  }

   }