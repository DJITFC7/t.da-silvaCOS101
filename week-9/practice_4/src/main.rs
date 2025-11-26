use std::fs::OpenOptions;
use::std::io::Write;

fn main() {
    
    let mut file = OpenOptions::new().append(true).open("../practice_1/data.txt").expect("file can't be opened");
    file.write_all("\nHello Class".as_bytes()).expect("Write failed");
    file.write_all("\nThis is the appendage to the document.".as_bytes()).expect("write failed");
    println!("filed append success");
}
