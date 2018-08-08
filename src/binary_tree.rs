use grid::Direction;
use grid::Grid;
use rand::Rng;
use rand::thread_rng;
use std::collections::HashSet;

pub fn binary_tree(grid: &mut Grid) -> HashSet<(i16, i16)> {
    let mut rng = thread_rng();
    let mut result = HashSet::new();
    for cell in grid.cells() {
        let neighbors = &[Direction::NORTH, Direction::EAST].iter()
            .filter_map(|d| grid.neighbor(cell, *d))
            .collect::<Vec<_>>();
        if let Some(target) = rng.choose(&neighbors) {
            result.insert((cell.id(), target.id()));
            result.insert((target.id(), cell.id()));
        }
    }
    result
}
