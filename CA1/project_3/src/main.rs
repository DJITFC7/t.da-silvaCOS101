use std::io;

fn main() {
    let mut input1 = String::new();
    let mut input2 = String::new();
    let mut input3 = String::new();
 
   println!("Cafe menu\n");
   println!("T = 800\n");
   println!("C = 1200\n");
   println!("S = 2000\n");
   println!("J = 1500\n");
 
   println!("Enter in item code");
    io::stdin().read_line(&mut input1).expect("entered a wrong input");


    println!("Enter in quantity");
    io::stdin().read_line(&mut input2).expect("entered a wrong input");
    let q:f32 = input2.trim().parse().expect("not a valid number");

    println!("Do u wish to add more? (y) or (n)");
    if input3 == "y"{
        continue;
    }else if input3 == "n"{
        break;
    }else {
        println!("please enter 'y' or 'n' to continue or stop");
    }


    
}
