fn main() {
    let mut count = 0;

    loop {
        count += 1;
        println!("Count: {}", count);

        if count == 7 {
            break;
        }
    }
    println!("Loop ended count at {}", count);
}