use std::io;
fn main() {
  // loop to repeat the process..
  loop {

    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    let mut input_4 = String::new();
    //Collect employees name
      println!("Dear employee, please input your name.");
    io::stdin().read_line(&mut input_1).expect("not a valid name");
    println!("You are welcome, {} ",input_1);

    println!("Dear {} are you experienced?\ntrue or false",input_1);
    io::stdin().read_line(&mut input_2).expect("out of line");
    let experience:bool = input_2.trim().parse().expect("not a bool");

//When the user choose his answer truthfully... the incentive shows immediately
    if !experience {
        println!("your annual incentive is: N100,000");
    }
    else{
     println!("what is your age?");
     io::stdin().read_line(&mut input_3).expect("Not a valid age");
     let age:u8 = input_3.trim().parse().expect("invalid integer");

    if age >= 40 {
        println!("Dear {} your age is {} so your annual incentive is: N1,560,000",input_1,age);
    }
     else if age < 40 && age >= 30 {
        println!("Dear {} your age is {} so your annual incentive is: N1,480,000",input_1,age);
     }
     else if age < 30 && age >= 28 {
        println!("Dear {} your age is {} so your annual incentive is: N1,300,000",input_1,age);
     }
     // This runs when the users age is not up to the experience age
     else{
         println!("Dear {} you are not experienced", input_1);
         println!("Therefore your annual incentive is: N100,000");
     }

    }
    //asking the user if he wants to continue the process
    println!("Do you want to continue ?\n Yes to continue  No to Quit");
    io::stdin().read_line(&mut input_4).expect("invalid option");
    //sometimes, the user might give an un even letter case, this converts it to lower case for easy acess
    let value = input_4.trim().to_lowercase(); 
    if value == "yes" {
        continue;
    }
    else if value == "no" {
        println!("Thank you for your time");
        break;
    }
   // when the user enters the answer that is not in the option
    else{
        println!("Please input yes or no");
    }
   
  }
}