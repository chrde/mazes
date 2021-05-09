use crate::{common::*, Step};
use rand::prelude::StdRng;
use rand::Rng;

pub fn generate(rng: &mut ThreadRng, width: usize, height: usize) -> Maze {
    let mut maze = Maze::new(width, height);
    let mut current_group = 1usize;
    for cell in 0..maze.len() {
        let east = maze.neighbor_at(cell, Neighbor::East);
        let north = maze.neighbor_at(cell, Neighbor::North);
        let finish_group = match (east, north) {
            (Some(_), Some(_)) => rng.gen_range(0..2) == 0,
            (Some(_), None) => false,
            (None, Some(_)) => true,
            (None, None) => true,
        };
        if finish_group {
            if north.is_some() {
                let rand = rng.gen_range(0..current_group);
                maze.link(cell - rand, Neighbor::North);
            }
            current_group = 1;
        } else {
            current_group += 1;
            maze.link(cell, Neighbor::East);
        }
    }
    maze
}
