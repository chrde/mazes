use crate::{common::*, Step};
use rand::prelude::StdRng;
use rand::Rng;

pub fn generate(rng: &mut StdRng, width: usize, height: usize) -> (Maze, Vec<Step>) {
    let mut maze = Maze::new(width, height);
    let mut current_group = 1usize;
    let mut steps = vec![];
    let mut last_direction = None;
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
                steps.push(Step::Direction {
                    old: last_direction,
                    new: Neighbor::North,
                });
                steps.push(Step::Link(cell - rand, Neighbor::North));
                last_direction = Some(Neighbor::North);
            }
            current_group = 1;
        } else {
            current_group += 1;
            let new = Neighbor::East;
            maze.link(cell, new);
            steps.push(Step::Direction {
                old: last_direction,
                new,
            });
            steps.push(Step::Link(cell, new));
            last_direction = Some(new);
        }
    }
    (maze, steps)
}
