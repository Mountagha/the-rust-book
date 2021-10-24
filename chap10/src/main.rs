use  std::fmt::Display;

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest{
            largest = item;
        }
    }
    largest
}

struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x > self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest number is y = {}", self.y);
        }
    }

}
fn main() {
    let number_list = vec![10, 40, 5, 100, 80];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['c', 'a', 'x', 'b'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

    let pair = Pair {
        x: 10,
        y: 15,  
    };
    println!("cordinates are {} and {}", pair.x, pair.y);
    pair.cmp_display();

}

