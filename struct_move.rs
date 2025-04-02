struct Owner {
    name: String,
}

struct House {
    owner: Owner,
    rooms: u32,
}

fn main() {
    let owner1 = Owner {
        name: String::from("Alice"),
    };

    let house1 = House {
        owner: owner1,
        rooms: 3,
    };

    println!("House owner: {}", house1.owner.name);
    println!("Number of rooms: {}", house1.rooms);
}