fn main(){
    // a list of nos
    let v = vec![100, 200, 300];
    borrow_vector(&v); 
    println!("Printing the value of main() x[0]={}", v[0]);
    println!("----------------------------------");
}

fn borrow_vector(z:&Vec<i32>){
    println!("---------------------------");
    println!("Inside print_vector function {:?} \n", z);
    println!("---------------------------");
}