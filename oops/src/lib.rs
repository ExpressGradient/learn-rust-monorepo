pub struct AveragedCollection {
    collection: Vec<i32>,
    average: f64
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.collection.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let pop_result = self.collection.pop();

        match pop_result {
            Some(value) => {
                self.update_average();
                Some(value)
            },
            None => None
        }
    }

    pub fn get_average(self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.collection.iter().sum();
        self.average = total as f64 / self.collection.len();
    }
}