fn main() {
    let mut x = 5;
    let y = &mut x; // y is a mutable reference to x
    let z = &mut x; // z is ALSO a mutable reference to x
    *y += 1; 
    *z += 1; // This will likely cause a data race since both y and z point to the same memory location
    println!("x = {}", x);
}