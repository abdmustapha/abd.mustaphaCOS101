use std::fs::File;
use std::io::Write;

fn main() {

    let commissioners = [
        "Aigbogun Alamba Daudu",
        "Murtala Afeez Bendu",
        "Okorocha Calistus Ogbonna",
        "Adewale Jimoh Akanbi",
        "Osazuwa Faith Etiye",
    ];

    let ministries = [
        "Internal Affairs",
        "Justice",
        "Defense",
        "Power and Steel",
        "Petroleum",
    ];

    let zones = [
        "South West",
        "North East",
        "South South",
        "South West",
        "South East",
    ];

    let mut file = File::create("efcc_merged.txt").expect("Failed to create file");

    for b in 0..commissioners.len() {

        let record = format!(
            "Commissioner: {},                 Ministry: {},                  Zone: {}\n",
            commissioners[b],
            ministries[b],
            zones[b]
        );

        file.write_all(record.as_bytes()).expect("Failed to write to file");
    }

    println!("EFCC datasets merged successfully!");
    println!("Check efcc_merged.txt for the results.");
}
