use std::io;

fn main() {


    let mut inp1 = String::new();
    println!("Enter employee age");
    io::stdin().read_line(&mut inp1).expect("not a valid string");
    let age:i32 = inp1.trim().parse().expect("not a valid number");

    let mut experience_input = String::new();
    println!("Is the employee experienced? (yes/no)");
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience_input = experience_input.trim().to_lowercase();

  if experience_input == "yes"{
    println!("you are experienced!");
  }else if experience_input == "no"{
    println!("you are not experienced");
  }else{
    println!("please enter 'yes' or 'no'");
  }

    if experience_input == "yes" && age >= 40 {
            println!("incentive = 1_560_000");
        }else if experience_input == "yes" && age >= 30 && age < 40{
            println!("incentive = 1_480_000");
        }else if experience_input == "yes" && age < 28 {
            println!("incentive = 1_300_000");
        }else if  experience_input ==  "yes" && age >= 28 && age < 30 {
          println!("incentive = 1_300_000");
    }else if experience_input == "no"{
         println!("incentive = 100_000");
}
}
