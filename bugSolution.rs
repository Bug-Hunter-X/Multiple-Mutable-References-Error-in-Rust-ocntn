fn main() {
    let mut x = 5;
    { // Create a new scope for one mutable reference
        let y = &mut x;
        *y = 6; // only modify x in this scope
    }
    let z = x; // create a copy to modify
    let mut z = z;  // make it mutable 
    z = 7; //modify the copy
    println!("x = {}", x);
    println!("z = {}", z);
}