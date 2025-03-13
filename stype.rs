fn main() {
    // string literal
    let mut s = "hello";
    println!("{}", s);

    // re-assign the mutable variable s
    s = "world"
    println!("{}", s);

    // string object
    let mut sobj = String::from("hello");
    println!("{}", sobj);

    sobj.push_str(", world!");
    println!("{}", sobj);
}