struct Artist{}
struct Teacher{}

trait Mensch{
    fn  think(&self) -> &'static str;
}

impl  Mensch for Artist{
    fn think(&self)-> &'static str{
        "phantastische gedanken...."
    }
}
impl Mensch for Teacher{
    fn think(&self)-> &'static str{
        "mach das aber am besten sooo!"
    }
}
fn return_a_random_mensch(rand_num:u8)->Box<dyn Mensch>{
    if rand_num>128{
        Box::new(Artist{})
    } else {
        Box::new(Teacher{})
    }
}
fn main() {
    let rand_number = 100;
    let mensch = return_a_random_mensch(rand_number);
    println!("der mensch denkt: {}", mensch.think());
}
