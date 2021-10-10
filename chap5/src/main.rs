#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32
}
impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }

}

fn main() {

    let rectangle = Rectangle{
        width: 25,
        height:10
    };

    let rectangle2 = Rectangle{
        width: 25,
        height:20
    };
    let square = Rectangle::square(10);
    println!("square {:?}", square);
    println!("the area of {:?} is {}", rectangle, rectangle.area());
    println!("is {:?} fits inside {:?} : {}", rectangle, rectangle2, rectangle.can_hold(&rectangle2));
}
