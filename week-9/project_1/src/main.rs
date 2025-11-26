use::std::Write;
use::std::Read;


fn main() {
    let larger:[&str] = ["33_Export","Desperados","Goldberg","Gulder","Heineken","Star" ];
    let stout:[&str] = ["Legend", "Turbo King", "Williams"];
    let non_alchohlic:[&str] = ["Maltina", "Amstel malta","Malta Gold", "Fayrouz"];
    let mut file = std::fs::File::create("High quality drinks")
    .expect("create failed");
    file.write_all("\nLarger: 33_Export, Desperados, Goldberg, Gulder, Heineken, Star").as_bytes()).expect("write failed");
    file.write_all("\nstou: Legend, Turbo king., Williams."); 
    println!("Hello, world!");
}
