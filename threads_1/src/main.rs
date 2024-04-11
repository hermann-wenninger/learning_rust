use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;

fn main() {
    let (sender, empfaenger) = channel();
    let mut v = vec![22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2,3,1,2,3,22,1,2];
   let handle = thread::spawn(move|| {
        let mut i = 0;
        let vi = 21;
        sender.send(vi).unwrap();
        for x in 1..55 {
            sender.send(x).unwrap();
            println!("Hallo Zahl aus dem erzeugten Strang----{}----{}!",i,i);
            //v[x]+=111;
            thread::sleep(Duration::from_nanos(1));
            i+=1;
            
        }
        //drop(v);
        //println!("{:?}",v);
       
    }

);

handle.join().unwrap();
    for i in 1..50 {
        let empf = empfaenger.try_recv().unwrap();
        println!("Erhalten: {:?}", empf);
        println!("Hallo Zahl {} aus dem thread", empf);
        thread::sleep(Duration::from_nanos(3));
    }
   
}