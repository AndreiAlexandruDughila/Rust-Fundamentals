

fn main() {
    let s1 = String::from("hello");

    let (mut s2, mut len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);

    s2 = String::from("x");
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}
