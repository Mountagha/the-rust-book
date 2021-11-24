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
