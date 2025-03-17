fn main() {
    let s1 = String::from("Hello");
    let s2 = s1; // s1 is moved to s2
    // println!("{}", s1);
    println!("{}", s2);
}