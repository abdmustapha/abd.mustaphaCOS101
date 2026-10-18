use std::io;

fn check_level(title: &str) {
    let aps1 = vec!["intern", "paralegal", "placement"];
    let aps2 = vec![
        "administrator",
        "research assistant",
        "junior associate",
        "classroom teacher",
    ];
    let aps3 = vec![
        "senior administrator",
        "phd candidate",
        "associate",
        "associate lawyer",
        "senior teacher",
    ];
    let el1 = vec![
        "office manager",
        "post-doc researcher",
        "senior associate",
        "leading teacher",
    ];
    let el2 = vec!["director", "senior lecturer", "deputy principal"];
    let ses = vec!["ceo", "dean", "partner", "principal"];

    if aps1.contains(&title) {
        println!("Level: APS 1-2");
        println!("Years: 1 to 2 years");
     } 


     else if aps2.contains(&title) {
        println!("Level: APS 3-5");
        println!("Years: 3 to 5 years");
    } 
    

    else if aps3.contains(&title) {
        println!("Level: APS 5-8");
        println!("Years: 5 to 8 years");
    } 

    else if el1.contains(&title) {
        println!("Level: EL1");
        println!("Years: 8 to 10 years");
    } 

    
    else if el2.contains(&title) {
        println!("Level: EL2");
        println!("Years: 10 to 13 years");
    }


     else if ses.contains(&title) {
        println!("Level: SES");
        println!("Years: Top level");
    }

     else {
        println!("Please input a valuid title ");
    }
}

fn main() {
    let mut title = String::new();

    println!("Available titles:
     intern, paralegal, placement, administrator,\n \nresearch assistant, junior associate, classroom teacher, senior administrator, 
     \nphd candidate, associate, associate lawyer, senior teacher, 
     \noffice manager, post-doc researcher, senior associate, leading teacher, 
     \ndirector, senior lecturer, deputy principal, ceo, dean, partner, principal");

    println!("Enter job title:");

    io::stdin().read_line(&mut title).expect("Input error");

    let title = title.trim().to_lowercase();

    check_level(&title);
}
