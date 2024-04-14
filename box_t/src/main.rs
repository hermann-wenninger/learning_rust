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
}
