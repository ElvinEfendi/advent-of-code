use std::collections::HashMap;

fn has_adjacent_symbol(engine_schematic: &Vec<Vec<u8>>, i: usize, j: usize) -> bool {
    let directions = [(0, -1), (-1, -1), (-1, 0), (-1, 1),
                      (0, 1), (1, 1), (1, 0), (1, -1)];

    for (di, dj) in directions {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && ni < engine_schematic.len() as i32
            && nj >= 0 && nj < engine_schematic[0].len() as i32 {

            let el = engine_schematic[ni as usize][nj as usize];

            if el != b'.' && !el.is_ascii_digit() {
                return true;
            }
        }
    }

    false
}

fn sum_of_part_numbers(engine_schematic: &Vec<Vec<u8>>) -> u32 {
    let width = engine_schematic[0].len();
    let height = engine_schematic.len();
    let mut part_number: u32 = 0;
    let mut is_part_number = false;
    let mut sum: u32 = 0;

    for i in 0..height {
        for j in 0..width {
            if engine_schematic[i][j].is_ascii_digit() {
                part_number = part_number * 10 + (engine_schematic[i][j] - b'0') as u32;

                if !is_part_number {
                    is_part_number = has_adjacent_symbol(&engine_schematic, i, j);
                }
            }
            if !engine_schematic[i][j].is_ascii_digit() || j == width - 1 {
                if is_part_number {
                    sum += part_number;
                }
                part_number = 0;
                is_part_number = false;
            }
        }
    }

    sum
}

fn adjacent_gear_symbol_coordinates(engine_schematic: &Vec<Vec<u8>>, i: usize, j: usize) -> Option<(usize, usize)> {
    let directions = [(0, -1), (-1, -1), (-1, 0), (-1, 1),
                      (0, 1), (1, 1), (1, 0), (1, -1)];

    for (di, dj) in directions {
        let ni = i as i32 + di;
        let nj = j as i32 + dj;

        if ni >= 0 && ni < engine_schematic.len() as i32
            && nj >= 0 && nj < engine_schematic[0].len() as i32 {

            let el = engine_schematic[ni as usize][nj as usize];

            if el == b'*' {
                return Some((ni as usize, nj as usize));
            }
        }
    }

    None
}

fn sum_of_gear_ratios(engine_schematic: &Vec<Vec<u8>>) -> u32 {
    let width = engine_schematic[0].len();
    let height = engine_schematic.len();
    let mut part_number: u32 = 0;
    let mut gear_coordinates: Vec<(usize, usize)> = Vec::new();
    let mut gear_to_part_number: HashMap<(usize, usize), Vec<u32>> = HashMap::new();

    for i in 0..height {
        for j in 0..width {
            if engine_schematic[i][j].is_ascii_digit() {
                part_number = part_number * 10 + (engine_schematic[i][j] - b'0') as u32;

                if gear_coordinates.is_empty() {
                    if let Some((gi, gj)) = adjacent_gear_symbol_coordinates(&engine_schematic, i, j) {
                        gear_coordinates.push((gi, gj));
                    }
                }
            }
            if !engine_schematic[i][j].is_ascii_digit() || j == width - 1 {
                if !gear_coordinates.is_empty() {
                    for (gi, gj) in &gear_coordinates {
                        gear_to_part_number.entry((gi.clone(), gj.clone())).or_insert(Vec::new()).push(part_number);
                    }
                    gear_coordinates.clear();
                }
                part_number = 0;
            }
        }
    }

    gear_to_part_number.values()
        .filter(|part_numbers| part_numbers.len() == 2)
        .map(|part_numbers| part_numbers.iter().product::<u32>())
        .sum()
}

fn main() {
    let engine_schematic: Vec<Vec<u8>> = include_str!("../input.txt")
        .lines()
        .map(|line| line.as_bytes().to_vec())
        .collect();

    let sum = sum_of_part_numbers(&engine_schematic);
    println!("Sum of part numbers: {}", sum);

    let sum = sum_of_gear_ratios(&engine_schematic);
    println!("Sum of gear ratios: {}", sum);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_adjacent_symbol() {
        let text = "\
467..114..
?.........
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

        let engine_schematic: Vec<Vec<u8>> = text
            .lines()
            .map(|line| line.as_bytes().to_vec())
            .collect();

        assert_eq!(has_adjacent_symbol(&engine_schematic, 0, 0), true);
        assert_eq!(has_adjacent_symbol(&engine_schematic, 0, 1), true);
        assert_eq!(has_adjacent_symbol(&engine_schematic, 0, 2), false);

        assert_eq!(has_adjacent_symbol(&engine_schematic, 0, 5), false);
        assert_eq!(has_adjacent_symbol(&engine_schematic, 0, 6), false);
        assert_eq!(has_adjacent_symbol(&engine_schematic, 0, 7), false);
    }
}
