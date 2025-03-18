fn main() {
    let s1 = gives_ownership(); // s1 takes ownership of the returned string
    let s2 = String::from("Hello");
    let s3 = takes_and_gives_back(s2); // s2 is moved to the function and ownership is returned to s3

    // println!("{}", s2); // Error: s2 is no longer valid
    println!("{}", s3);
}

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // Return the string, transferring ownership to the caller
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // Return the string, transferring ownership back to the caller
}