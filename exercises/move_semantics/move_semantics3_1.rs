// move_semantics3.rs
// Make me compile without adding new lines-- just changing existing lines!
// (no lines with multiple semicolons necessary!)
// Execute `rustlings hint move_semantics3` for hints :)

// I AM NOT DONE

fn main() {
    let mut vec0 = Vec::new();
    vec0.push(22);

    let (ov, on) = fill_vec2(&vec0);

    let vec1: Vec<String> = Vec::new();

    println!("{} has length {} content `{:?}`", "vec1", vec0.len(), vec0);
    println!("{} has length {} content `{:?}`", "ov", ov.len(), ov);
    println!("{} has length {} content `{:?}`", "on", on.len(), on);
    //vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}


fn fill_vec2(vec: &Vec<i32>) -> (&Vec<i32>,Vec<i32>) {
    println!("vec is {:?}", vec);
    let mut vec1 = vec.clone();
    vec1.push(32);
    (vec, vec1)
}
