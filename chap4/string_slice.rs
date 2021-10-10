fn string_slice(to_slice: &String) -> &str {
    let bytes = to_slice.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' '{
            return &to_slice[0..i];
        }
    }
    &to_slice[..] 
}

fn main() {
    let s = String::from("slice this");
    println!("{}", string_slice(&s));
}
