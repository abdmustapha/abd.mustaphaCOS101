//Creating a quadratic calculator
use std::io;
fn main() {
    //collect the users value for the quadratic equaation
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    
    //tells user to input their value and then convert to float
     println!("Enter value [a] to continue");
    io::stdin().read_line(&mut input_1).expect("invalid string");
    let a:f64 = input_1.trim().parse().expect("invalid number");
     
    println!("Enter value [b] to continue");
     io::stdin().read_line(&mut input_2).expect("invalid string");
    let b:f64 = input_2.trim().parse().expect("invalid number");
     
    println!("Enter value [c] to continue");
     io::stdin().read_line(&mut input_3).expect("invalid string");
    let c:f64 = input_3.trim().parse().expect("invalid number");

    if a  == 0.0 {
        println!("This is not a quadratic equation but a linear equation");
    }
      
   else {
       //i break the quadratic formula so the compiler can understand the information and instruction i am trying to pass
    let determinant = b*b - 4.0 *a *c;
    
    if determinant >=0.0 {
    let root = determinant.sqrt();

    let nominator_1 = (-b + root)/ (2.0*a);

    let nominator_2 = (-b - root)/ (2.0*a);

     println!("answer: {} or {}", nominator_1, nominator_2);
    }
    //After creatin the calculator, i noticed it wasnt calculating some set of numbers due to complex solution... i had AI break down the formula, to correct this.
    else{
        let imaginary = (-determinant).sqrt();
        let real = -b / (2.0 * a);
        let imaginary_part = imaginary / (2.0 * a);
        println!("answer: {} or {}", real, imaginary_part);
    }
   }
   
}