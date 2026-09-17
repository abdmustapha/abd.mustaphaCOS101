use std::io;

fn main() {
    let mut lower_bound = String::new();
    let mut upper_bound = String::new();

     println!("Enter lower bound");
    io::stdin().read_line(&mut lower_boundary).expect("invalid string");
    let lower_bound:i32 = input1.trim().parse().expect("In valid number");

    println!("Enter upper bound");
    io::stdin().read_line(&mut input2).expect("In valid String");
    let upper_bound:i32 = input2.trim().parse().expect("Invalid number");

    for x in lower_bound..upper_bound{
        println!("count level {}",x );
    }
}
