use std::io;

fn main() {

    println!("Is the employee experienced? (yes/no):");
    let mut experience_input = String::new();
    io::stdin().read_line(&mut experience_input).expect("Failed to read input");
    let experience_input = experience_input.trim().to_lowercase();

    let mut inp1 = String::new();
    println!("Enter employee age");
    io::stdin().read_line(&mut inp1).expect("not a valid string");
    let age:i32 = inp1.trim().parse().expect("not a valid number");

   
   let incentive: i32;

    if experience_input == "yes" {
        if age >= 40 {
            incentive = 1_560_000;
        }else if age >= 30 && age < 40{
            incentive = 1_480_000;
        }else if age < 28 {
            incentive = 1_300_000;
        }else {
          incentive = 1_300_000;
        }
    }else {
        incentive = 100_000;
    }
    println!("The annual incentive for the employee is {} ", incentive);
}
