//! Chap14
//! `chap14 is a collection of utilities to make performing certain
//! calculations more convenients

/// Adds one to the number given
/// 
/// # Examples
/// 
/// ```
/// let arg = 5;
/// let answer = chap14::add_one(arg);
/// 
/// assert_eq!(6, answer);
/// ```

pub fn add_one(x: i32) -> i32 {
    x + 1
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn add_one_test() {
        assert_eq!(add_one(3), 4);
    }
}
