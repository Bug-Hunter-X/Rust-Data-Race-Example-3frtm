use std::sync::{Arc, Mutex};

fn main() {
    let x = Arc::new(Mutex::new(5));
    
    // Clone the Arc to allow multiple threads to access x
    let y = x.clone();
    let z = x.clone();

    let handle1 = std::thread::spawn(move || {
        let mut x_locked = y.lock().unwrap();
        *x_locked += 1;
    });

    let handle2 = std::thread::spawn(move || {
        let mut x_locked = z.lock().unwrap();
        *x_locked += 1;
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    println!("x = {}", *x.lock().unwrap());
}