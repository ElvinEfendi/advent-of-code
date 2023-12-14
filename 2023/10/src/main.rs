use day_10::grid;

fn main() {
    let input = include_str!("../input.txt");
    let grid = grid::Grid::from(input);
    println!("Distance to farthest cell: {}", grid.distance_to_farthest_cell());
}
