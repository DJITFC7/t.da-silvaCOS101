use std::io;

fn main() {
     let mut input_1 = String::new();
     let mut input_2 = String::new();
     let mut input_3 = String::new();
     let mut user_input = String::new();
    println!("Enter principle");
    io::stdin().read_line(&mut input_1).expect("not a valid input");
    let p:f32 = input_1.trim().parse().expect("not a valid number");

    println!("Enter in rate");
    io::stdin().read_line(&mut input_2).expect("not a valid input");
    let r:f32 = input_2.trim().parse().expect("not a valid number");
let t = loop {
     println!("Enter in time");
    io::stdin().read_line(&mut input_3).expect("not a valid input");
    let t:f32 = input_3.trim().parse().expect("not a valid number");
    match input_3.trim().parse::<f32>(){
        Ok(num) => break num,
        Err(_) => println!("please enter in a number"),
    }


    let a = p * (1.0 + r/100.0) .powf(t);
    let i = a + p;
    println!("the amount and interest = {} and {}", a, i);
    
     let decision = loop{
    println!("Do you want to calculate for another borrower? 'y' or 'n'\n");
    io::stdin().read_line(&mut user_input).expect("not a valid input");
    match user_input.trim().parse::<char>(){
         Ok(char) => break char,
         Err(_) => println!("please enter 'y' or 'n'"),
     }
    
   
    if user_input == "y" {
        continue;
    }else if user_input == "n" {
        println!("Exiting loop....");
        break n;
        
    }else {
        println!("please enter 'y' to continue or 'n' to stop");
    }
    };
};
    }
