
use std::io;

fn main() {
    let r = "Poundo Yam & Edinkaiko Soup";
    let s = "Fried Rice & Chicken";
    let t = "Amala & Ewedu Soup";
    let u = "Eba & Egusi Soup";
    let v = "White Rice & Soup";

    let p: u32 = 3200;
    let f: u32 = 3000;
    let a: u32 = 2500;
    let e: u32 = 2000;
    let w: u32 = 2500;

    loop {
        let mut input_1 = String::new();
        let mut input_2 = String::new();

        println!("\nDear Customer, Welcome To PAU Restaurant Menu!");
        println!("P == Poundo Yam & Edinkaiko Soup");
        println!("F == Fried Rice & Chicken");
        println!("A == Amala & Ewedu Soup");
        println!("E == Eba & Egusi Soup");
        println!("W == White Rice & Soup");
        println!("Choose from P F A E W:");

        io::stdin()
            .read_line(&mut input_1)
            .expect("Not a valid option!!!");

        input_1 = input_1.trim().to_lowercase();

        if input_1 == "p" {
            println!("How many packages of {} do you want?", r);
            io::stdin()
                .read_line(&mut input_2)
                .expect("Not a valid option!!!");

            let package: u32 = input_2.trim().parse().expect("Input a valid integer");
            let price = p * package;

            println!("Total price is {}", price);

            if price > 10000 {
                let discount = price * 5 / 100;
                let final_price = price - discount;

                println!("Discount is {}", discount);
                println!("Final price is {}", final_price);
            } else {
                println!("No discount.");
            }
        }

        else if input_1 == "f" {
            println!("How many packages of {} do you want?", s);
            io::stdin()
                .read_line(&mut input_2)
                .expect("Not a valid option!!!");

            let package: u32 = input_2.trim().parse().expect("Input a valid integer");
            let price = f * package;

            println!("Total price is {}", price);

            if price > 10000 {
                let discount = price * 5 / 100;
                let final_price = price - discount;

                println!("Discount is {}", discount);
                println!("Final price is {}", final_price);
            } else {
                println!("No discount.");
            }
        }

        else if input_1 == "a" {
            println!("How many packages of {} do you want?", t);
            io::stdin()
                .read_line(&mut input_2)
                .expect("Not a valid option!!!");

            let package: u32 = input_2.trim().parse().expect("Input a valid integer");
            let price = a * package;

            println!("Total price is {}", price);

            if price > 10000 {
                let discount = price * 5 / 100;
                let final_price = price - discount;

                println!("Discount is {}", discount);
                println!("Final price is {}", final_price);
            } else {
                println!("No discount.");
            }
        }

        else if input_1 == "e" {
            println!("How many packages of {} do you want?", u);
            io::stdin()
                .read_line(&mut input_2)
                .expect("Not a valid option!!!");

            let package: u32 = input_2.trim().parse().expect("Input a valid integer");
            let price = e * package;

            println!("Total price is {}", price);

            if price > 10000 {
                let discount = price * 5 / 100;
                let final_price = price - discount;

                println!("Discount is {}", discount);
                println!("Final price is {}", final_price);
            } else {
                println!("No discount.");
            }
        }

        else if input_1 == "w" {
            println!("How many packages of {} do you want?", v);
            io::stdin()
                .read_line(&mut input_2)
                .expect("Not a valid option!!!");

            let package: u32 = input_2.trim().parse().expect("Input a valid integer");
            let price = w * package;

            println!("Total price is {}", price);

            if price > 10000 {
                let discount = price * 5 / 100;
                let final_price = price - discount;

                println!("Discount is {}", discount);
                println!("Final price is {}", final_price);
            } else {
                println!("No discount.");
            }
        }

        else {
            println!("Please enter only from the options provided.");
            continue;
        }

        // Ask whether the customer wants to order again
        let mut again = String::new();

        println!("\nDo you want to order again? (Y/N)");

        io::stdin()
            .read_line(&mut again)
            .expect("Could not read input");

        again = again.trim().to_lowercase();

        if again != "y" {
            println!("Thank you for ordering from PAU Restaurant!");
            break;
        }
    }
}