pub struct History {
    values: Vec<isize>,
}

impl History {
    fn new(values: Vec<isize>) -> Self {
        Self { values }
    }

    pub fn predict_next_value(&self) -> isize {
        let mut next_values = self.values.clone();
        let mut next_value = 0;

        loop {
            next_value += next_values[next_values.len() - 1];

            if next_values.iter().all(|v| v == &0) {
                break;
            }

            next_values = next_values
                .windows(2)
                .map(|w| {
                    w[1] - w[0]
                })
                .collect::<Vec<_>>();
        }

        next_value
    }
}

impl From<&str> for History {
    fn from(s: &str) -> Self {
        let values = s
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect::<Vec<_>>();

        Self::new(values)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_history_from_str() {
        let history = History::from("0 3 6 9 12 15");

        assert_eq!(history.values, vec![0, 3, 6, 9, 12, 15]);
    }

    #[test]
    fn test_history_predict_next_value() {
        let history = History::from("0 3 6 9 12 15");
        assert_eq!(18, history.predict_next_value());

        let history = History::from("1 3 6 10 15 21");
        assert_eq!(28, history.predict_next_value());

        let history = History::from("10 13 16 21 30 45");
        assert_eq!(68, history.predict_next_value());
    }
}
