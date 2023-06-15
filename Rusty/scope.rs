fn take_ownership(s: String) {
    println!("{}", s);
}

fn main() {
    let s = String::from("XX");
    take_ownership(s);
    println!("{}", s);
}