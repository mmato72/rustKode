struct Rectangle {
    width: u32,
    height: u32,
}

fn modify_dimensions(rect: &mut Rectangle) {
    rect.width = 40;
    rect.height = 60;
}

fn main() {
    let mut rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    modify_dimensions(&mut rect1); // Mutable borrow

    println!("Modified Width: {}", rect1.width);
    println!("Modified Height: {}", rect1.height);
}