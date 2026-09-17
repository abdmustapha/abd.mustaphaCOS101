/* Rust program to calculate the ara of a triangle for a given base and height*/
 use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    
    println!("Please input the value of your base");
    io::stdin().read_line(&mut input_1).expect("Not a valid string");
    let b:f32 = input_1.trim().parse().expect("not a valid number");
     
    println!("Please input the value of your height"); 
    io::stdin().read_line(&mut input_2).expect("Not a valid string");
    let h:f32 = input_2.trim().parse().expect("not a valid number");

    let area = (b * h)/2.0;
    

    if area >0.0 {
      println!("area equals: {}", area);
    }
}
