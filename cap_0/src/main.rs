#![doc(html_playground_url = "https://play.rust-lang.org/")]
//! ```
//! println!("Hello Wolrd and universe");
//! ```

use std::io;
fn main() {
    println!("Rate eien zahl zwischen 0 und 100");

    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("fehler beim lesen");
    println("SCHÄTZUNG ... {guess}")

}
