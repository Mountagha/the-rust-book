#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two (a: i32) -> i32 {
    a + 2
}

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 0 and 100 got {}", value);
        }
        Guess { value }
    }
}


#[cfg(test)]
mod tests {

    use super::*; 

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cant_hold_larger() {
        let larger = Rectangle {
            width: 10,
            height: 10,
        };
        let smaller = Rectangle {
            width: 5,
            height: 6,
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn test_add_two() {
        assert_eq!(add_two(5), 7);
        assert_ne!(add_two(5), 5);
    }

    #[test]
    #[should_panic]
    fn test_guess(){
        Guess::new(300);
    }
}
