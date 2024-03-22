#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let x = String::from("asdfghjkl");
    let b = Box::new(x);
    println!("b in box = {}", b);
    
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Cons(4, Box::new(Cons(5, Box::new(Nil))))))))));
   
    println!("{:#?}",list);
}
