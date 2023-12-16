/*
- => (i, j+1)     ( 0,  1)
| => (i+1, j)     ( 1,  0)
7 => (i-1, j-1)   (-1, -1)
L => (i+1, j+1)   ( 1,  1)
J => (i+1, j-1)   ( 1, -1)
F => (i-1, j+1)   (-1, +1)
*/
use std::collections::HashSet;

#[derive(PartialEq, Debug)]
enum Connection {
    NorthSouth,
    EastWest,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
}

impl Connection {
    fn directions(&self) -> [(i32, i32); 2] {
        match self {
            Connection::NorthSouth => [(1, 0), (-1, 0)],
            Connection::EastWest => [(0, -1), (0, 1)],
            Connection::NorthEast => [(-1, 0), (0, 1)],
            Connection::NorthWest => [(-1, 0), (0, -1)],
            Connection::SouthWest => [(0, -1), (1, 0)],
            Connection::SouthEast => [(0, 1), (1, 0)],
        }
    }
}

#[derive(PartialEq, Debug)]
enum Cell {
    Ground,
    Animal,
    Pipe(Connection),
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    animal_position: (usize, usize),
    polygon: Vec<(usize, usize)>,
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut cells = Vec::new();
        let mut animal_position = (0, 0);

        for (i, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                let cell = match c {
                    '.' => Cell::Ground,
                    'S' => {
                        animal_position = (i, j);
                        Cell::Animal
                    }
                    '|' => Cell::Pipe(Connection::NorthSouth),
                    '-' => Cell::Pipe(Connection::EastWest),
                    'L' => Cell::Pipe(Connection::NorthEast),
                    'J' => Cell::Pipe(Connection::NorthWest),
                    '7' => Cell::Pipe(Connection::SouthWest),
                    'F' => Cell::Pipe(Connection::SouthEast),
                    _ => panic!("Invalid character: {}", c),
                };
                row.push(cell);
            }
            cells.push(row);
        }

        let polygon = Vec::new();
        Self { cells, animal_position, polygon }
    }
}

impl Grid {
    fn cell_connection_at(&self, position: (usize, usize), direction: (i32, i32)) -> Option<(usize, usize)> {
        let to = (
            (position.0 as i32 + direction.0) as usize,
            (position.1 as i32 + direction.1) as usize,
        );

        if to.0 >= self.cells.len() || to.1 >= self.cells[0].len() {
            return None;
        }

        let cell = &self.cells[to.0][to.1];

        match cell {
            Cell::Pipe(pipe) => {
                match direction {
                    (0, 1) => {
                        match pipe {
                            Connection::EastWest | Connection::SouthWest | Connection::NorthWest => {
                                Some(to)
                            }
                            _ => None
                        }
                    }
                    (1, 0) => {
                        match pipe {
                            Connection::NorthSouth | Connection::NorthEast | Connection::NorthWest => {
                                Some(to)
                            }
                            _ => None
                        }
                    }
                    (-1, 0) => {
                        match pipe {
                            Connection::NorthSouth | Connection::SouthWest | Connection::SouthEast => {
                                Some(to)
                            }
                            _ => None
                        }
                    }
                    (0, -1) => {
                        match pipe {
                            Connection::EastWest | Connection::SouthEast | Connection::NorthEast => {
                                Some(to)
                            }
                            _ => None
                        }
                    }
                    _ => None
                }
            }
            Cell::Animal => {
                Some(to)
            }
            _ => None
        }
    }

    fn cell_connections(&self, position: (usize, usize)) -> [(usize, usize); 2] {
        let mut result = [(0, 0); 2];
        let mut index = 0;

        let directions = match &self.cells[position.0][position.1] {
            Cell::Animal => [(0, 1), (1, 0), (-1, 0), (0, -1)],
            Cell::Pipe(connection) => {
                let dir = connection.directions();
                [dir[0], dir[1], (0, 0), (0, 0)]
            }
            _ => return result,
        };

        for direction in directions.iter() {
            match self.cell_connection_at(position, *direction) {
                Some(to) => {
                    result[index] = to;
                    index += 1;
                }
                None => continue,
            };
        }

        result
    }

    pub fn distance_to_farthest_cell(&mut self) -> usize {
        let mut distance = 1;
        let mut routes = self.cell_connections(self.animal_position);
        let mut previous_routes = [self.animal_position, self.animal_position];
        let mut polygon_second_half = Vec::new();

        self.polygon.clear();
        self.polygon.push(self.animal_position);

        loop {
            self.polygon.push(routes[0]);
            if routes[0] == routes[1] {
                break;
            } else {
                polygon_second_half.insert(0, routes[1]);
            }

            let new_route_0 = *self.cell_connections(routes[0]).iter().filter(|&c| c != &previous_routes[0]).next().unwrap();
            let new_route_1 = *self.cell_connections(routes[1]).iter().filter(|&c| c != &previous_routes[1]).next().unwrap();

            previous_routes = routes;

            routes[0] = new_route_0;
            routes[1] = new_route_1;

            distance += 1;
        }

        self.polygon.extend(polygon_second_half);

        distance
    }

    fn is_point_in_polygon(&self, p: (usize, usize)) -> bool {
        let mut intersections = 0;
        let (xp, yp) = (p.0 as f64, p.1 as f64);

        for i in 0..self.polygon.len() {
            let (x1, y1) = (self.polygon[i].0 as f64, self.polygon[i].1 as f64);
            let (x2, y2) = (self.polygon[(i + 1) % self.polygon.len()].0 as f64, self.polygon[(i + 1) % self.polygon.len()].1 as f64);

            if (yp < y1) != (yp < y2) && xp < x1 + ((yp-y1) / (y2-y1)) * (x2-x1) {
                intersections += 1;
            }
        }

        intersections % 2 == 1
    }

    pub fn count_enclosed_points(&self) -> usize {
        let mut count = 0;
        let mut polygon_lookup = HashSet::new();

        for p in self.polygon.iter() {
            polygon_lookup.insert(p);
        }

        for i in 0..self.cells.len() {
            for j in 0..self.cells[i].len() {
                if self.is_point_in_polygon((i, j)) {
                    if !polygon_lookup.contains(&(i, j)) {
                        count += 1;
                    }
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_from_str() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let grid = Grid::from(input);

        assert_eq!(5, grid.cells.len());
        assert_eq!(5, grid.cells[0].len());
        assert_eq!(Cell::Ground, grid.cells[0][0]);
        assert_eq!(Cell::Pipe(Connection::SouthEast), grid.cells[0][2]);
        assert_eq!(Cell::Pipe(Connection::NorthWest), grid.cells[4][1]);
        assert_eq!(Cell::Animal, grid.cells[2][0]);
        assert_eq!((2, 0), grid.animal_position);
    }

    #[test]
    fn test_grid_cell_connections() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let grid = Grid::from(input);

        assert_eq!([(2, 1), (3, 0)], grid.cell_connections((2, 0)));
        assert_eq!([(1, 1), (2, 0)], grid.cell_connections((2, 1)));
    }

    #[test]
    fn test_grid_distance_to_farthest_cell() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let mut grid = Grid::from(input);
        assert_eq!(8, grid.distance_to_farthest_cell());

        let input = "\
.....
.S-7.
.|.|.
.L-J.
.....";
        let mut grid = Grid::from(input);
        assert_eq!(4, grid.distance_to_farthest_cell());

        let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let mut grid = Grid::from(input);
        assert_eq!(4, grid.distance_to_farthest_cell());
        assert_eq!(
            vec![(1, 1), (1, 2), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (2, 1)],
            grid.polygon,
        );
    }

    #[test]
    fn test_grid_is_point_in_polygon() {
        let input = "\
..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        let mut grid = Grid::from(input);
        assert_eq!(8, grid.distance_to_farthest_cell());
        assert_eq!(
            vec![
                (2, 0), (2, 1), (1, 1), (1, 2), (0, 2), (0, 3),
                (1, 3), (2, 3), (2, 4), (3, 4), (3, 3), (3, 2), (3, 1), (4, 1), (4, 0), (3, 0),
            ],
            grid.polygon,
        );
        assert_eq!(false, grid.is_point_in_polygon((2, 0)));
        assert_eq!(false, grid.is_point_in_polygon((1, 0)));
        assert_eq!(true, grid.is_point_in_polygon((2, 2)));

        let input = "\
-L|F7
7S-7|
L|7||
-L-J|
L|-JF";
        let mut grid = Grid::from(input);
        assert_eq!(4, grid.distance_to_farthest_cell());
        assert_eq!(
            vec![(1, 1), (1, 2), (1, 3), (2, 3), (3, 3), (3, 2), (3, 1), (2, 1)],
            grid.polygon,
        );
        assert_eq!(true, grid.is_point_in_polygon((2, 2)));

        let input = "\
.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        let mut grid = Grid::from(input);
        assert_eq!(70, grid.distance_to_farthest_cell());
        assert_eq!(true, grid.is_point_in_polygon((4, 7)));
        assert_eq!(false, grid.is_point_in_polygon((0, 0)));
    }
}
