#[derive(Debug)]
pub struct Race {
    max_time: u32,
    best_distance: u32,
}

impl Race {
    pub fn new(max_time: u32, best_distance: u32) -> Self {
        Self {
            max_time,
            best_distance,
        }
    }

    /*
    When the button is held for n milliseconds, the speed will n millimeters per millisecond.
    It also eats up from the total race time. Thus, given n milliseconds button held time,
    the distance covered would be calculated as d = (max_time - n) * n; That means,
    the the distance has parabolic relation with the button held time. So by plugging best_distance
    in there we can calculate the possible button held times by solving the quadratic equation:
    (max_time - n) * n = best_distance => n^2 - max_time * n + best_distance = 0.
    After solving the equation, and obtaining n1 and n2, then we can say that
    there are floor(n2) - ceil(n1) + 1 possible button held times
    where it'd beat best distance (because the parabola is concave).
     */
    pub fn number_of_beating_held_times(&self) -> u32 {
        let discriminant = (self.max_time as i32).pow(2) - 4 * self.best_distance as i32;
        if discriminant < 0 {
            panic!("No possible button held time");
        }

        let sqrt_discriminant = (discriminant as f64).sqrt();
        let n1 = (self.max_time as f64 - sqrt_discriminant) / 2.0;
        let n2 = (self.max_time as f64 + sqrt_discriminant) / 2.0;

        ((n2 - 1.0).ceil() - (n1 + 1.0).floor() + 1.0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_beating_held_times() {
        let race = Race {
            max_time: 7,
            best_distance: 9,
        };
        assert_eq!(4, race.number_of_beating_held_times());

        let race = Race {
            max_time: 15,
            best_distance: 40,
        };
        assert_eq!(8, race.number_of_beating_held_times());

        let race = Race {
            max_time: 30,
            best_distance: 200,
        };
        assert_eq!(9, race.number_of_beating_held_times())
    }

    #[test]
    #[should_panic]
    fn test_number_of_beating_held_times_panics() {
        let race = Race {
            max_time: 7,
            best_distance: 900000,
        };
        let _ = race.number_of_beating_held_times();
    }
}
