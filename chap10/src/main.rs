fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}
fn main() {
    let number_list = vec![10, 40, 5, 100, 80];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['c', 'a', 'x', 'b'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
