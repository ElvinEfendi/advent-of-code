use day_10::grid;

fn main() {
    let input = include_str!("../input.txt");
    let mut grid = grid::Grid::from(input);
    println!("Distance to farthest cell: {}", grid.distance_to_farthest_cell());

    println!("Number of enclosed cells: {}", grid.count_enclosed_points());
}
