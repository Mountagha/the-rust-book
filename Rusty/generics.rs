
fn largest(list: &[i32]) -> &i32 {
    let mut largest = &list[0];
    for number in list {
        if number > largest {
            largest = number;
        }
    }
    largest
}

fn main() {
    let numbers = vec![3, 4, 31, 2];
    let largest = largest(&numbers);
    println!("largest number is {}", largest)
}