use super::MazeGenerator;
use crate::{
    maze::{Maze, Neighbor1},
    render::{render_borders, render_cell, RED},
};
use rand::prelude::{IteratorRandom, StdRng};

#[derive(Copy, Clone, Debug)]
enum Step {
    Empty,
    Walk(usize),
    Link(usize, Neighbor1),
    Finished,
}

pub struct HuntAndKillGen {
    maze: Maze,
    steps: Vec<Step>,
    next: usize,
}

impl HuntAndKillGen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            maze: Maze::new(width, height),
            next: 0,
            steps: vec![Step::Empty],
        }
    }

    /// Go to a non-yet visited neighbor
    pub fn unvisited_neighbor(&self, rng: &mut StdRng, cell: usize) -> Option<Neighbor1> {
        self.maze
            .neighbors(cell)
            .iter()
            .filter(|n| self.maze.cell(n.idx).links().is_empty())
            .choose(rng)
    }

    /// Go to an already visited neighbor
    pub fn neighbor_with_link(&self, rng: &mut StdRng, cell: usize) -> Option<(usize, Neighbor1)> {
        self.maze
            .neighbors(cell)
            .iter()
            .filter(|n| self.maze.cell(n.idx).links().is_partial())
            .choose(rng)
            .map(|n| (cell, n))
    }
}

impl MazeGenerator for HuntAndKillGen {
    fn next(&mut self, rng: &mut StdRng) {
        let next = match self.steps[self.next] {
            Step::Empty => Step::Walk(0),
            Step::Walk(cell) => {
                if let Some(n) = self.unvisited_neighbor(rng, cell) {
                    Step::Link(cell, n)
                } else {
                    let candidate = self
                        .maze
                        .cells()
                        .iter()
                        .enumerate()
                        .filter(|(_, cell)| cell.links().is_empty())
                        .find_map(|(idx, _)| self.neighbor_with_link(rng, idx));
                    match candidate {
                        Some((cell, next)) => Step::Link(cell, next),
                        None => Step::Finished,
                    }
                }
            }
            Step::Link(cell, next) => {
                self.maze.link(cell, next.dir);
                Step::Walk(next.idx)
            }
            Step::Finished => {
                return;
            }
        };
        if self.next == self.steps.len() - 1 {
            println!("running {:?}", self.steps[self.next]);
            self.steps.push(next);
        } else {
            println!("replay {:?}", self.steps[self.next]);
            // replay
        }
        self.next += 1;
    }

    fn prev(&mut self) {
        if self.next == 0 {
            return;
        }
        self.next -= 1;
        println!("undoing {:?}", self.steps[self.next]);
        match self.steps[self.next] {
            Step::Empty => {}
            Step::Walk(_) => {}
            Step::Link(cell, next) => self.maze.unlink(cell, next.dir),
            Step::Finished => {}
        }
    }

    fn finished(&self) -> bool {
        matches!(self.steps[self.next], Step::Finished)
    }

    fn completed(&self) -> bool {
        self.steps
            .last()
            .map_or(false, |s| matches!(s, Step::Finished))
    }

    fn steps_count(&self) -> usize {
        self.steps.len()
    }

    fn next_step(&self) -> usize {
        self.next
    }

    fn maze(&self) -> &Maze {
        &self.maze
    }

    fn render(&mut self, render_group: &mut host_api::RenderGroup, border_color: host_api::Color) {
        for y in 0..self.maze().height() {
            for x in 0..self.maze().width() {
                render_borders(render_group, x, y, self.maze(), border_color);
            }
        }
        match self.steps[self.next] {
            Step::Empty => {}
            Step::Walk(cell) => {
                let x = cell % self.maze.width();
                let y = cell / self.maze.width();
                render_cell(render_group, x, y, RED)
            }
            Step::Link(cell, _) => {
                let x = cell % self.maze.width();
                let y = cell / self.maze.width();
                render_cell(render_group, x, y, RED)
            }
            Step::Finished => {}
        }
    }
}
