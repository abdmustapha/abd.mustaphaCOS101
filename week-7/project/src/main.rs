use std::io;

fn main(){
    let mut input_1 = String::new();

     println!("\n\nDEAR USER... Welcome to the Original Casio Calculator...\n To calculate the area, volume or surface area, chooose from the below option\n
        \nT - For trapezium 
        \nR - For rhombus
        \nP - For area_of_parallelogram
        \nC - For cube_surface_area
        \nV - For cylinder_volume
        ");
    
    io::stdin().read_line(&mut input_1).expect("Not a string");
    let input_1 = input_1.trim().to_lowercase(); 

    if input_1 == "t" {
        let result = area_of_trapezium();
        println!("The area of your trapezium is: {}", result);
    }
    else if input_1 == "r" {
        let result = area_of_rhombus();
        println!("The area of your rhombus is: {}", result);
    }
    else if input_1 == "p" {
        let result = area_of_parallelogram();
        println!("The area of your parallelogram is: {}", result);
    }
    else if input_1 == "c" {
        let result = cube_surface_area();
        println!("The surface area of your cube is: {}", result);
    }
    else if input_1 == "v" {
        let result = cylinder_volume();
        println!("The volume of your cylinder is: {}", result);
     }
     else {
        println!("your selection has exceeded the scope of this calculator");
     }   
}

fn area_of_trapezium() -> f32 {
    let mut input_2 = String::new();
    let mut input_3 = String::new();
    let mut input_4 = String::new();

    println!("To calculate the area of your trapezium, \n input height...");
    io::stdin().read_line(&mut input_2).expect("this is not an integer");
    let h:f32 = input_2.trim().parse().unwrap();

    println!("Input the base 1 height.");
    io::stdin().read_line(&mut input_3).expect("this is not an integer");
    let b1:f32 = input_3.trim().parse().unwrap();

    println!("Input the base 2 height.");
    io::stdin().read_line(&mut input_4).expect("this is not an integer");
    let b2:f32 = input_4.trim().parse().unwrap();

    h / 2.0 * (b1 + b2)
}

fn area_of_rhombus() -> f32 {
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!("To calculate the area of your rhombus, \n input diagonal 1...");
    io::stdin().read_line(&mut input_2).expect("this is not an integer");
    let d1:f32 = input_2.trim().parse().unwrap();

    println!("Input diagonal 2 to continue.");
    io::stdin().read_line(&mut input_3).expect("this is not an integer");
    let d2:f32 = input_3.trim().parse().unwrap();

    0.5 * d1 * d2
}

fn area_of_parallelogram() -> f32 {
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!("To calculate the area of your parallelogram, \n input base...");
    io::stdin().read_line(&mut input_2).expect("this is not an integer");
    let b:f32 = input_2.trim().parse().unwrap();

    println!("Input altitude value to continue.");
    io::stdin().read_line(&mut input_3).expect("this is not an integer");
    let a:f32 = input_3.trim().parse().unwrap();

    b * a
}

fn cube_surface_area() -> f32 {
    let mut input_2 = String::new();

    println!("To calculate the surface area of your cube, \n input the length ...");
    io::stdin().read_line(&mut input_2).expect("this is not an integer");
    let l:f32 = input_2.trim().parse().unwrap();

    6.0 * l * l
}

fn cylinder_volume() -> f32 {
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!("To calculate the volume of your cylinder, \n input radius...");
    io::stdin().read_line(&mut input_2).expect("this is not an integer");
    let r:f32 = input_2.trim().parse().unwrap();

    println!("Input the height of your cylinder to continue.");
    io::stdin().read_line(&mut input_3).expect("this is not an integer");
    let h:f32 = input_3.trim().parse().unwrap();

    3.141592654 * r * r * h
}