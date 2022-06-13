// move_semantics6.rs
// Make me compile! `rustlings hint move_semantics6` for hints
// You can't change anything except adding or removing references


fn main() {
    let mut data = "Rust is great!".to_string();

    get_char(&data);

    rm_char(&mut data);

    string_uppercase(&data);
}

// Should not take ownership
fn get_char(mut data: &String) -> char {
    data.chars().last().unwrap()
}

// Remove last char
fn rm_char(mut data: &mut String) -> String {
    data.pop();
    data.to_string()

}

// Should take ownership
fn string_uppercase(mut data: &String) {
    let data = &data.to_uppercase();

    println!("{}", data);
}
