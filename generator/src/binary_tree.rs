use crate::{common::*, Step};
use rand::prelude::StdRng;
use rand::Rng;

pub fn generate(rng: &mut StdRng, width: usize, height: usize) -> (Maze, Vec<Step>) {
    let mut maze = Maze::new(width, height);
    let mut steps = vec![];
    let mut last_direction = None;
    for cell in 0..maze.len() {
        let n: Vec<_> = [Neighbor::North, Neighbor::East]
            .iter()
            .cloned()
            .filter(|n| maze.neighbor_at(cell, *n).is_some())
            .collect();
        if !n.is_empty() {
            let choice = n[rng.gen_range(0..n.len())];
            steps.push(Step::Direction {
                old: last_direction,
                new: choice,
            });
            maze.link(cell, choice);
            steps.push(Step::Link(cell, choice));
            last_direction = Some(choice);
        }
    }
    (maze, steps)
}
