use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();

  
   println!("Cafe menu\n");
   println!("T = £800\n");
   println!("C = £1200\n");
   println!("S = £2000\n");
   println!("J = £1500\n");
 
   println!("Enter in item code");
    io::stdin().read_line(&mut input1).expect("entered a wrong input");
   let code_input:String = input1.trim().parse().expect("not a valid input");

    println!("Enter in quantity");
    io::stdin().read_line(&mut input2).expect("entered a wrong input");
    let q:u16 = input2.trim().parse().expect("not a valid number");
    
    let te = q * 800;
    let co = q * 1_200;
    let sp = q * 2_000;
    let ju = q * 1_500;
  // let input1 = input1.to_uppercase();
  if code_input== "T" {
    println!("You have picked Tea (£800)");
    println!("cost is: {}",te);
  }  else if code_input == "C"{
    println!("You have picked Coffee (£1200)");
    println!("cost is: {}",co);
  }else if code_input == "S" {
    println!("You have picked Sprite (£2000)");
    println!("cost is: {}",sp);
  }else if code_input == "J"{
    println!("You have picked Juice");
    println!("cost is: {}",ju);
  }

 }
