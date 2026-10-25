use  std::io::Write;
use std::io;
use  std::io::Read;
fn main() {

        
    println!("=======================================================================");
    println!("             PAU Student Management Information System");
    println!("=======================================================================");

    let vec_1 = vec! [
         ("Oluchi Mordi  ", "  Ac10211111", "  Accounting" , 300),
         ("Adams Aliyu  ", "   Eco10110101", "  Economics", 100),
         ("Shania Bolade  ", "  csc10328828", "  Computer", 200),
         ("Adekunle Gold  ", "  EEE11020202", "  Electronics", 200),
         ("Blanca Edemoh  ", "  MEE11202001", "  Mechanical", 100),
    ];
    let mut vec_3:Vec<String> = Vec::new();

    for vec_1 in &vec_1 {
      let vec_2 = format!("{},{},{},{}\n",
        vec_1.0, 
        vec_1.1, 
        vec_1.2, 
        vec_1.3
      );
      vec_3.push(vec_2)
    };

    let mut file = std::fs::File::create("vec_3.csv").expect("create failed");

    for line in &vec_3 {
    file.write_all(line.as_bytes()).expect("Write failed");
}
     println!("==============================");
     println!("\nData written to file");
     println!("==============================");

    let mut file = std::fs::File::open("vec_3.csv").expect("Open failed");

    let mut contents = String::new();

    file.read_to_string(&mut contents).expect("Read failed");

    println!("Data inside the file:\n");
    println!("{}", contents);
    



}    