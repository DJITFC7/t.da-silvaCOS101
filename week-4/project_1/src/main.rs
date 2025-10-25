use std::io;

fn main() {

   let mut input1 = String::new();
   let mut input2 = String::new();
   let mut input3 = String::new();

    println!("enter coefficient of x^2");
     io::stdin().read_line(&mut input1).expect("not a valid string");
     let a:i32 = input1.trim().parse().expect("not a valid number");

    println!("enter coefficient of x");
     io::stdin().read_line(&mut input2).expect("not a valid string");
     let b:i32 = input2.trim().parse().expect("not a valid number");

    println!("enter ");
     io::stdin().read_line(&mut input3).expect("not a valid string");
     let c:i32 = input3.trim().parse().expect("not a valid number");


    let d = b^2 - (4 * a * c);

    if d > 0
    {
        println!("roots are distinct");
    }
    if d < 0 
    {
        println!("roots are imaginary");
    }
    if d == 0
    {
        println!("roots are equal and real");
    }


}

  
