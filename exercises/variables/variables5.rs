// variables5.rs
// Make me compile! Execute the command `rustlings hint variables5` if you want a hint :)

// using shadow variables https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#shadowing

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3;
    println!("Number plus two is : {}", number + 2);
}
