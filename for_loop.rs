fn main() {
    for num in 1..10 {
        if num == 5 {
            println!("Breaking at number: {}", num);
            break;
        }
        if num % 2 == 0 {
            continue;
        }
        println!("Number: {}", num);
    }
}