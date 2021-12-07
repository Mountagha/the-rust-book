pub struct Averagedcollection {
    list: Vec<i32>,
    average: f64,
}

impl Averagedcollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn new() -> Averagedcollection {
        Averagedcollection { list: vec![], average: 0.0 }
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        match result {
            Some(value) => {
                self.update_average();
                Some(value)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.average
    }
    
    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }

}
// blog post added cause i'm lazy to create new project.

pub struct Post {
    state: Option<Box<dyn State>>,
    content: String,
}

impl Post {
    pub fn new() -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }

    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }

    pub fn content(&self) -> &str {
        ""
    }

    pub fn request_review(&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.request_review())
        }
    }


}

trait State {
    fn request_review(self: Box<self>) -> Box<dyn State>;
}

struct Draft {}

impl State for Draft {
    fn request_review(self: box<self>) -> Box<dyn State> {
        Box::new(PendingReview {})
    }
}

struct PendingReview {}

impl State for PendingReview {
    fn request_review(self: Box<self>) -> box<dyn State> {
        self
    }
}



#[cfg(test)]
mod tests {

    use super::*;


    #[test]
    fn test_average() {
        let mut aC = Averagedcollection::new();
        assert_eq!(aC.average(), 0.0);
        aC.add(5); 
        aC.add(4); 
        aC.add(1); 
        aC.add(2); 
        assert_eq!(aC.average(), 3.0);
    }
}
