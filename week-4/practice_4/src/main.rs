use std::io;

fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();

  println!("Enter your name");
  io::stdin().read_line(&mut input_1).expect("Not a valid string");

  println!("Enter your age");
  io::stdin().read_line(&mut input_2).expect("Not a valid integer");
  let age:u16 = input_2.trim().parse().expect("Not a valid number");

  if age >= 18 {
    println!("Welcome to the party {}",input_1);
  }
  else{
    print!("oops, you are not of age to enter the party {}",input_1);
  }
}
