fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let my_string = String::from("hello world");

    // `first_word` работает с фрагментами `String`, как частичными, так и целыми
    let word = first_word(&my_string[0..6]);
    println!("{word}");
    let word = first_word(&my_string[..]);
    println!("{word}");
    // `first_word` также работает со ссылками на `String`, которые эквивалентны
    // к целым фрагментам `String`.
    let word = first_word(&my_string);
    println!("{word}");

    let my_string_literal = "hello world";

    // `first_word` works on slices of string literals, whether partial or whole
    let word = first_word(&my_string_literal[0..6]);
    println!("{word}");
    let word = first_word(&my_string_literal[..]);
    println!("{word}");

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let word = first_word(my_string_literal);
    println!("{word}");
}
