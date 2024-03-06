//possibility to take slice
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
fn main() {
    let fullsentence = String::from("Soviel von Sowas?");
    let word = first_word(&fullsentence);
    println!("{}",&fullsentence[ ..word]);
    let go = fullsentence;
    println!("{}",&go[..2]);
}
