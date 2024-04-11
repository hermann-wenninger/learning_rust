#[warn(dead_code)]
struct Santa {
    
    color: String,
    phrase: String,
}
#[warn(dead_code)]
struct Rudolph {

    speed: u32,
    lumens: u32,
}

fn new_santa() -> Santa {
    Santa {
        color: String::from("Red"),
        phrase: String::from("Ho ho ho!"),
    }
}
fn new_rudolph() -> Rudolph {
    Rudolph {
        speed: 100,
        lumens: 500,
    }
}

enum Common {
  Santa(Santa),
  Rudolph(Rudolph),
}

fn main() {
    let santa = new_santa();
    let rudolph = new_rudolph();
    let x = new_santa();

    let mut northerners = vec![];
     northerners.push(Common::Santa(santa));
     northerners.push(Common::Rudolph(rudolph));
     northerners.push(Common::Santa(x));
    match &northerners[0] {
        Common::Santa(santa) => println!("santa color: {}", santa.color),
        Common::Rudolph(rudolph) => println!("rudolph speed: {}", rudolph.speed),
    }
    
}