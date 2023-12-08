/*
Example input, games:

Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green

Cubes in the bag: 12 red cubes, 13 green cubes, and 14 blue cubes
 */

use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash, Clone)]
enum Cube {
    Red,
    Green,
    Blue,
}

struct Round {
    cubes: HashMap<Cube, u32>,
}

struct Game {
    total_cubes: HashMap<Cube, u32>,
    id: u32,
    rounds: Vec<Round>,
}

impl From<&str> for Round {
    fn from(s: &str) -> Self {
        let mut cubes = HashMap::new();
        for c in s.split(", ") {
            let mut iter = c.split_whitespace();
            let count = iter.next().unwrap().parse::<u32>().unwrap();
            let cube = match iter.next().unwrap() {
                "red" => Cube::Red,
                "green" => Cube::Green,
                "blue" => Cube::Blue,
                _ => panic!("Invalid color"),
            };
            cubes.insert(cube, count);
        }
        Self { cubes }
    }
}

impl From<&str> for Game {
    fn from(s: &str) -> Self {
        let mut iter = s.split(": ");

        let id = iter.next().unwrap().split_whitespace().nth(1).unwrap().parse::<u32>().unwrap();

        let mut rounds = Vec::new();
        let rounds_str = iter.next().unwrap();
        for r in rounds_str.split("; ") {
            rounds.push(Round::from(r));
        }

        let total_cubes = [
            (Cube::Red, 12),
            (Cube::Green, 13),
            (Cube::Blue, 14),
        ].iter().cloned().collect();

        Self { total_cubes, id, rounds }
    }
}

impl Game {
    fn is_valid(&self) -> bool {
        for r in &self.rounds {
            for (cube, count) in &r.cubes {
                let max_available_count = self.total_cubes.get(cube).unwrap();
                if count > max_available_count {
                    return false;
                }
            }
        }
        true
    }
}

fn main() {
    let sum_of_valid_game_ids = include_str!("../input.txt")
        .lines()
        .map(|l| Game::from(l))
        .filter(|g| g.is_valid())
        .map(|g| g.id)
        .sum::<u32>();

    println!("Sum of valid game IDs: {}", sum_of_valid_game_ids);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_round_from() {
        let r = Round::from("3 blue, 4 red");

        assert_eq!(r.cubes.get(&Cube::Blue).unwrap(), &3);
        assert_eq!(r.cubes.get(&Cube::Red).unwrap(), &4);
    }

    #[test]
    fn test_round_from_all_colors() {
        let r = Round::from("3 blue, 4 red, 5 green");

        assert_eq!(r.cubes.get(&Cube::Blue).unwrap(), &3);
        assert_eq!(r.cubes.get(&Cube::Red).unwrap(), &4);
        assert_eq!(r.cubes.get(&Cube::Green).unwrap(), &5);
    }

    #[test]
    #[should_panic(expected = "Invalid color")]
    fn test_round_from_invalid_str() {
        let _ = Round::from("3 blue, 4 red, 5 green, 6 yellow");
    }

    #[test]
    fn test_game_from() {
        let g = Game::from("Game 5: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert_eq!(g.id, 5);
        assert_eq!(g.rounds.len(), 3);
        assert_eq!(g.rounds[0].cubes.get(&Cube::Blue).unwrap(), &3);
        assert_eq!(g.rounds[0].cubes.get(&Cube::Red).unwrap(), &4);
        assert_eq!(g.rounds[1].cubes.get(&Cube::Red).unwrap(), &1);
        assert_eq!(g.rounds[1].cubes.get(&Cube::Green).unwrap(), &2);
        assert_eq!(g.rounds[1].cubes.get(&Cube::Blue).unwrap(), &6);
        assert_eq!(g.rounds[2].cubes.get(&Cube::Green).unwrap(), &2);
    }

    #[test]
    fn test_game_is_valid() {
        let g = Game::from("Game 5: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green");

        assert!(g.is_valid());
    }

    #[test]
    fn test_game_is_not_valid() {
        let g = Game::from("Game 5: 3 blue, 13 red; 1 red, 2 green, 6 blue; 2 green");

        assert!(!g.is_valid());
    }
}
