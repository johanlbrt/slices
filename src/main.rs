fn first_word(s: &String) {
    let bytes = s.as_bytes();
    println!("{:?}",bytes);
}

fn main() {
    let text = String::from("abcd ");
    first_word(&text);
}