struct Laptop {
    brand: String,
    price: u32,
}

fn cost(laptop: &Laptop, quantity: u32) -> u32 {
    laptop.price * quantity
}

fn main() {
    let laptops = vec![
        Laptop {
            brand: String::from("HP"),
            price: 650_000,
        },
        Laptop {
            brand: String::from("IBM"),
            price: 755_000,
        },
        Laptop {
            brand: String::from("Toshiba"),
            price: 650_000,
        },
        Laptop {
            brand: String::from("Dell"),
            price: 850_000,
        },
    ];

    let quantity = 3;
    let mut total = 0;

    for laptop in &laptops {
        let laptop_cost = cost(laptop, quantity);

        println!(
            "{} ({} units): ₦{}",
            laptop.brand, quantity, laptop_cost
        );

        total += laptop_cost;
    }

    println!("---------------------------------------");
    println!("Total cost for {} laptops from each brand: ₦{}",quantity, total);
}
