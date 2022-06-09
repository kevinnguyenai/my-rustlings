// variables2.rs
// Make me compile! Execute the command `rustlings hint variables2` if you want a hint :)


fn main() {
    let x: i32 = 0xFF;
    let y: i32 = 98_000;
    let z: usize = 0o77;
    let w: usize = 0b1111_0000;
    let b: u8 = b'A';
    if x == 255 {
        println!("Ten!");
        println!("y : {}",y);
        println!("z : {}",z);
        println!("w : {}",w);
        println!("b : {}",b);
    } else {
        println!("Not ten!");
    }
}
