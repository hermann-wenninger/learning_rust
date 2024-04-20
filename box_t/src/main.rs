fn main() {

  let mut x = 42;
 
  fn change(x:&mut u8)->&mut u8{
    *x +=42;
    x
    }

  change(&mut x);
  println!("{}",x);

  let mut counter = 0;
  let qu = loop{
    counter += 1;
    println!("loop number {}",counter);
    if counter == 100{
        counter *=2;
        break;
    }
  };
  println!("end {}",counter);

  println!("Größe eines Zeichens: {}", std::mem::size_of::<char>());
  println!("Größe von a: {}", "a".len());
  println!("Größe von ß: {}", "ß".len());
  println!("Size of国: {}", "国".len());
  println!("Größe von 𓅱 : {}", "𓅱".len());
  println!("{:?}", "a".as_bytes());
    println!("{:?}", "ß".as_bytes());
    println!("{:?}", "国".as_bytes());
    println!("{:?}", "𓅱".as_bytes());
 let bu = {
 struct User  { name: "Mr. User", user_number: 101 }
    
  };
  println!("{:?}",bu);
}
