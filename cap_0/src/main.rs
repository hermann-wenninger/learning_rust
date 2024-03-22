//Structures and Tupels
struct Person{
    name: String,
    vname:String,
    alter:u8,
}   
struct Tupelix (i8,f64,String);

#[derive(Debug)]
struct Point{
    x:f64,
    y:f64,
    z:f64,
}
struct Cube{
    hoehe:u32,
    breite:u32,
    tiefe:u32,
}
#[derive(Debug)]
struct CubeOnPoint{
    hoehe:u32,
    breite:u32,
    tiefe:u32,
    position: Point,
}
fn main() {
let nameintub = String::from_utf8("i am tub".into());
let tub = Tupelix(22,7.87,nameintub.expect("REASON"));
println!("{}..{}..{:#?}",tub.0,tub.1,&tub.2);
let cube = Cube {hoehe:23, breite:44, tiefe:99};
println!("h:{}----b:{}----t:{}",cube.hoehe,cube.breite,cube.tiefe);
let from_cube = Cube{hoehe:1, breite:999, tiefe:123};
println!("h:{}----b:{}----t:{}",from_cube.hoehe,from_cube.breite,from_cube.tiefe);
let xyz = Point{x:1.1,y:2.2,z:3.3};

println!("{:#?}",xyz);
let next_cube = CubeOnPoint{hoehe:1,breite:2,tiefe:3,position: xyz};
println!("cube  - {:#?}",next_cube);
}
