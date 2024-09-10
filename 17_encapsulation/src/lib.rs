pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
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
    fn can_add() {
        let mut ac = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        ac.add(1);
        ac.add(2);
        ac.add(3);
        ac.add(4);
        ac.add(5);
        ac.add(6);
        ac.add(7);
        ac.add(8);
        ac.add(9);
        ac.add(10);
    }

    #[test]
    fn can_remove() {
        let mut ac = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        ac.add(1);
        ac.add(2);
        ac.add(3);
        ac.add(4);
        ac.add(5);
        ac.add(6);
        ac.add(7);
        ac.add(8);
        ac.add(9);
        ac.add(10);

        ac.remove();
        ac.remove();
        ac.remove();
        ac.remove();
        ac.remove();
        ac.remove();
        ac.remove();
        ac.remove();
        ac.remove();
        let result = ac.remove();

        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn correct_average() {
        let mut ac = AveragedCollection {
            list: vec![],
            average: 0.0,
        };

        ac.add(1);
        ac.add(2);
        ac.add(3);
        ac.add(4);
        ac.add(5);
        ac.add(6);
        ac.add(7);
        ac.add(8);
        ac.add(9);
        ac.add(10);

        let avg = ac.average();
        assert_eq!(avg, 5.5);
    }
}
