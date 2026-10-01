fn main() {
    let name = "Aisha lawal";
    let uni:&str = "Pan_Atlantic University";
    let addr:&str = "Km 52 lekki-Epe Expressway, ibeju-lekki, Lagos";
    println!("Name: {}",name);
    println!("University: {}, \nAddress: {}",uni,addr);



    let department:&'static str = "Computer Science";
    let school:&'static str = "School of science andd technology";
    println!("Department: {}, \nSchool: {}", department,school);
}
