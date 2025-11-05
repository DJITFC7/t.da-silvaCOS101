fn main() {
  let name = "Aisha Lawal";
  let uni:&str = "Pan-Atlantic University";
  let addr:&str = "km 52 Lekki Expressway, Ibeju-Lekki, Lagos";
  println!("Name: {}",name );
  println!("UNiversity: {} \n Address: {}",uni, addr );

  let department:&'static str = "computer Science";
  let school:&'static str = "schoo of science of technology";
  println!("Depatment: {}, \nSchool: {}",department, school );
}
