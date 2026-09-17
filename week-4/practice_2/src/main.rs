use std::io;
fn main() {
    let mut input_1 = String::new();
    let mut input_2 = String::new();
    let mut input_3 = String::new();

    println!("Enter the first edge of your triangle");
    io::stdin()
    .read_line(&mut input_1)
    .expect("Not a valid string");
    let a:f32 = input_1.trim().parse().expect("not a valid number");

    println!("Enter the second edge of your triangle");
    io::stdin()
    .read_line(&mut input_2)
    .expect("Not a valid string");
    let b:f32 = input_2.trim().parse().expect("not a valid number");


    println!("Enter the third edge of your triangle");
    io::stdin()
    .read_line(&mut input_3)
    .expect("Enter the third edge of your triangle");
    let c:f32 = input_3.trim().parse().expect("not a valid number");

    
    let s:f32 = (a + b + c)/ 2.0;
    let mut area:f32 = s * (s - a) * (s - b) * (s - c);
    area = area.sqrt();

    println!("Area of triangle: {}", area);
}
