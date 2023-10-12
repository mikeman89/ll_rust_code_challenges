use std::fmt::Display;

fn info<T: Display>(text: &T) {
    println!("{}", text)
}

fn main() {
    let a = "?";
    let b = "?".to_owned();
    info(&a);
    info(&b);
}

#[test]
fn owned_string() {
    let input = "?".to_owned();
    info(&input);
}

#[test]
fn str_slice() {
    let input = "?";
    info(&input);
}
