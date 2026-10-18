fn main() {
    let applicants = vec![
        ("Faith", 3),
        ("Jeremiah", 5),
        ("Emanuel", 2),
        ("Abdulazeez", 8),
    ];

    let mut highest = applicants[0];

    for person in &applicants {
        if person.1 > highest.1 {
            highest = *person;
        }
    }

    println!("Most experienced programmer: {}", highest.0);
    println!("Years of experience: {}", highest.1);
}
