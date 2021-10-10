fn main() {
    //let mut s1 = String::from("hello");
    let s1 = takes_ownership_and_give_back(String::from("hello"));

    takes_ownership(s1);

    let x = 5;

    makes_move(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn takes_ownership_and_give_back(other_string: String) -> String {
    println!("owning and giving back {}", other_string);
    other_string
}

fn makes_move(some_integer: i32) {
    println!("{}", some_integer);
}