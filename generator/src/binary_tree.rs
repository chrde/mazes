use crate::common::*;
use rand::prelude::ThreadRng;
use rand::Rng;

pub fn generate(rng: &mut ThreadRng, width: usize, height: usize) -> Maze {
    let mut maze = Maze::new(width, height);
    for cell in 0..maze.len() {
        let n: Vec<_> = [Neighbor::North, Neighbor::East]
            .iter()
            .cloned()
            .filter(|n| maze.neighbor_at(cell, *n).is_some())
            .collect();
        if !n.is_empty() {
            let choice = n[rng.gen_range(0..n.len())];
            maze.link(cell, choice);
        }
    }
    maze
}
