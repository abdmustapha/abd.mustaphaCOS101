use std::io;

fn main() {
    loop {
        let mut input1 = String::new();
        println!("\nThe incentive calculator");
        println!("To calculate your incentive, enter 1;");
        println!("To exit, enter 2;");
        
        io::stdin().read_line(&mut input1).expect("Enter valid digit");
        let ca: u8 = input1.trim().parse().expect("Enter valid digit");

        if ca == 1 {
            let mut input2 = String::new();
            let mut input3 = String::new();

            println!("How old are you: ");
            io::stdin().read_line(&mut input2).expect("Enter valid digit");
            let age: u8 = input2.trim().parse().expect("Enter valid digit");

            println!("Are you experienced? true/false");
            io::stdin().read_line(&mut input3).expect("Enter valid digit");
            let exp: bool = input3.trim().parse().expect("Enter valid digit");

            if age > 40 && exp {
                println!("Your annual incentive is 1,560,000 naira");
            } else if age > 30 && exp {
                println!("Your annual incentive is 1,480,000 naira");
            } else if age > 18 && exp {
                println!("Your annual incentive is 1,300,000 naira");
            } else if age > 20 && !exp {
                println!("Your annual incentive is 100,000 naira");
            } else {
                println!("You are not qualified to be an employee");
            }
        } else if ca == 2 {
            break;
        }
    }
}

