#[derive(Debug)]
pub struct Race {
    max_time: f64,
    best_distance: f64,
}

impl Race {
    pub fn new(max_time: u64, best_distance: u64) -> Self {
        Self {
            max_time: max_time as f64,
            best_distance: best_distance as f64,
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

    For visual see: parabola.png in the root of the project.
     */
    pub fn number_of_beating_held_times(&self) -> u32 {
        let discriminant = self.max_time.powf(2.0) - 4.0 * self.best_distance;
        if discriminant < 0.0 {
            panic!("No possible button held time");
        }

        let sqrt_discriminant = discriminant.sqrt();
        let n1 = (self.max_time - sqrt_discriminant) / 2.0;
        let n2 = (self.max_time + sqrt_discriminant) / 2.0;

        ((n2 - 1.0).ceil() - (n1 + 1.0).floor() + 1.0) as u32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_beating_held_times() {
        let race = Race::new(7, 9);
        assert_eq!(4, race.number_of_beating_held_times());

        let race = Race::new(15, 40);
        assert_eq!(8, race.number_of_beating_held_times());

        let race = Race::new(30, 200);
        assert_eq!(9, race.number_of_beating_held_times());

        let race = Race::new(71530, 940200);
        assert_eq!(71503, race.number_of_beating_held_times());

    }

    #[test]
    #[should_panic]
    fn test_number_of_beating_held_times_panics() {
        let race = Race::new(7, 900000);
        let _ = race.number_of_beating_held_times();
    }
}
