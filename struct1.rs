struct Rectangle {
    width: u32,
    height: u32,
}


fn main() {
    let r1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("Width: {}", r1.width);
    println!("Height: {}", r1.height);
}