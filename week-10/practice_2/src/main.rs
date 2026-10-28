fn main() {
    let v = vec![10, 20, 30];
    let v2 = v;

    println!("In main {:?}", v2);

    display(v2);
}

fn display(v: Vec<i32>) {
    println!("Inside display {:?}", v);
}
