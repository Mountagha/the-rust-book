fn main() {
    let s1 = String::from("hello");
    
    takes_ownership(s1);

    let x = 5;

    makes_move(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_move(some_integer: i32) {
    println!("{}", some_integer);
}