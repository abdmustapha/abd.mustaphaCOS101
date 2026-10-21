use  std::io::Write;
fn main() {
    let details = "Lager, Stout, Non-Alcholic\n";
    let line_1 = "33Export, Turbo King,  Amstel Malta\n";
    let line_2 = "Goldberg, Williams, Malta Gold\n";
    let line_3 = "Guider, , , \n";
    let line_4 = "Star, , , \n";

    let mut file = std::fs::File::create("data.csv").expect("create failed");
    
    file.write_all(details.as_bytes()).expect("write failed");
    file.write_all(line_1.as_bytes()).expect("write failed");
    file.write_all(line_2.as_bytes()).expect("write failed");
    file.write_all(line_3.as_bytes()).expect("write failed");
    file.write_all(line_4.as_bytes()).expect("write failed");

    println!("\nData written to file");


}    