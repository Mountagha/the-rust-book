pub fn raindrops(n: u32) -> String {
    let mut result = String::new();
    let is_factor = |x| n % x == 0;
    let divisors = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    for (k, v) in divisors.iter() {
        if is_factor(k) {
            result.push_str(v);
        }
    }
    if result.is_empty() { return n.to_string(); }
    result
}

fn main() {
    println!("{}", raindrops(105));
}