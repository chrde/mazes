use super::MazeGenerator;
use crate::maze::{Maze, Neighbor, Neighbor1};
use rand::prelude::{IteratorRandom, StdRng};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
enum Step {
    Empty,
    Walk(usize),
    BackTrack,
    Link(usize, Neighbor1),
    Finished,
}

pub struct RecurBacktrackerGen {
    maze: Maze,
    next: usize,
    stack: Vec<usize>,
    completed: Vec<usize>,
    steps: Vec<Step>,
}

impl RecurBacktrackerGen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            stack: vec![],
            completed: vec![],
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
}

impl MazeGenerator for RecurBacktrackerGen {
    fn next(&mut self, rng: &mut StdRng) {
        let next = match self.steps[self.next] {
            Step::Empty => Step::Walk(0),
            Step::Walk(cell) => {
                if let Some(next) = self.unvisited_neighbor(rng, cell) {
                    Step::Link(cell, next)
                } else {
                    Step::BackTrack
                }
            }
            Step::BackTrack => match self.stack.pop() {
                Some(cell) => {
                    self.completed.push(cell);
                    Step::Walk(cell)
                }
                None => Step::Finished,
            },
            Step::Finished => {
                return;
            }
            Step::Link(cell, next) => {
                self.stack.push(cell);
                self.maze.link(cell, next.dir);
                Step::Walk(next.idx)
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
            Step::BackTrack => {
                if let Some(cell) = self.completed.pop() {
                    self.stack.push(cell)
                }
            }
            Step::Link(cell, next) => {
                self.stack.pop();
                self.maze.unlink(cell, next.dir)
            }
            Step::Finished => {}
        }
    }

    fn finished(&self) -> bool {
        matches!(self.steps[self.next], Step::Finished)
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

    fn completed(&self) -> bool {
        self.steps
            .last()
            .map_or(false, |s| matches!(s, Step::Finished))
    }
}
