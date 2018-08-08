use grid::Direction;
use grid::Grid;
use rand::Rng;
use rand::thread_rng;
use std::collections::HashSet;

pub fn sidewinder(grid: &mut Grid) -> HashSet<(i16, i16)> {
    let mut rng = thread_rng();
    let mut result = HashSet::new();
    let mut run = Vec::new();
    for cell in grid.cells() {
        let neighbors = &[Direction::NORTH, Direction::EAST].iter()
            .filter_map(|d| grid.neighbor(cell, *d).map(|c| (c, *d)))
            .collect::<Vec<_>>();
        if let Some((target, dir)) = rng.choose(&neighbors) {
            run.push(cell);
            let (s, t) = match dir {
                Direction::EAST => (cell.id(), target.id()),
                Direction::NORTH => {
                    let result = {
                        let cell = rng.choose(&run).expect("Current run is empty");
                        let neighbor = grid.neighbor(*cell, Direction::NORTH).unwrap();
                        (cell.id(), neighbor.id())
                    };
                    run.clear();
                    result
                }
                _ => unreachable!()
            };
            result.insert((s, t));
            result.insert((t, s));
        }
    }
    result
}
