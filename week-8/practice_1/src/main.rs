fn main() {
    //using Vec::new()
    let v:Vec<i64> = Vec::new();

    //printing the size of vector
    println!("The length of vec::new is :{}",v.len());
    
    //using macro
    let v = vec!["Grace", "Efiong", "Basil","Kareem","Susan"];
    //printing the size of the vector
    println!("The length of Vec macro is: {}",v.len());
}
