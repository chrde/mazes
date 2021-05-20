use crate::{
    common::*, gen::MazeGenerator, render::DARK_RED, render_borders, render_cell, Color,
    RenderGroup, RED,
};
use rand::prelude::{IteratorRandom, SliceRandom, StdRng};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum WilsonStep {
    Empty,
    StartWalk,
    RandDir(usize),
    Walk(usize, usize),
    EraseWalk(usize),
    FinishWalk(usize, usize),
    Link(usize, usize),
    Finished,
}

#[derive(Clone, Eq, PartialEq)]
pub struct WilsonGen {
    pub maze: Maze,
    pub unvisited: Vec<usize>,
    pub visited: Vec<usize>,
    pub links: Vec<Link>,
    pub current_walk: Vec<usize>,
    pub truncated_walk: Vec<usize>,
    pub steps: Vec<WilsonStep>,
    pub next: usize,
}

impl WilsonGen {
    pub fn new(rng: &mut StdRng, width: usize, height: usize) -> Self {
        let maze = Maze::new(width, height);
        let mut unvisited: Vec<_> = (0..maze.len()).into_iter().collect();
        unvisited.shuffle(rng);
        let current_walk = vec![];
        let steps = vec![WilsonStep::Empty];
        Self {
            maze,
            next: 0,
            unvisited,
            visited: vec![],
            links: vec![],
            truncated_walk: vec![],
            current_walk,
            steps,
        }
    }
}

impl MazeGenerator for WilsonGen {
    fn render(&mut self, render_group: &mut RenderGroup, border_color: Color) {
        for idx in &self.visited {
            let x = idx % self.maze.width();
            let y = idx / self.maze.width();
            render_cell(
                render_group,
                x,
                y,
                Color {
                    r: 34,
                    g: 70,
                    b: 70,
                },
            );
        }
        for idx in &self.unvisited {
            let x = idx % self.maze.width();
            let y = idx / self.maze.width();
            render_cell(render_group, x, y, Color { r: 31, g: 0, b: 0 });
        }
        for idx in &self.current_walk {
            let x = idx % self.maze.width();
            let y = idx / self.maze.width();
            render_cell(
                render_group,
                x,
                y,
                Color {
                    r: 120,
                    g: 80,
                    b: 105,
                },
            );
        }
        for y in 0..self.maze.height() {
            for x in 0..self.maze.width() {
                render_borders(render_group, x, y, &self.maze, border_color);
            }
        }
        match self.steps[self.next] {
            WilsonStep::Empty => {}
            WilsonStep::StartWalk => {}
            WilsonStep::RandDir(cell) => {
                let x = cell % self.maze.width();
                let y = cell / self.maze.width();
                render_cell(render_group, x, y, DARK_RED);
            }
            WilsonStep::Walk(cell, dir) => {
                let x = cell % self.maze.width();
                let y = cell / self.maze.width();
                render_cell(render_group, x, y, DARK_RED);
                let x = dir % self.maze.width();
                let y = dir / self.maze.width();
                render_cell(render_group, x, y, RED);
            }
            WilsonStep::EraseWalk(cell) => {
                let idx = self
                    .current_walk
                    .iter()
                    .position(|x| *x == cell)
                    .expect("cannot find cell in walk");
                for idx in &self.current_walk[idx..] {
                    let x = idx % self.maze.width();
                    let y = idx / self.maze.width();
                    render_cell(
                        render_group,
                        x,
                        y,
                        Color {
                            r: 210,
                            g: 130,
                            b: 208,
                        },
                    );
                }
            }
            WilsonStep::FinishWalk(cell, next) => {
                for idx in &self.current_walk {
                    let x = idx % self.maze.width();
                    let y = idx / self.maze.width();
                    render_cell(
                        render_group,
                        x,
                        y,
                        Color {
                            r: 34,
                            g: 110,
                            b: 110,
                        },
                    );
                }
                let x = cell % self.maze.width();
                let y = cell / self.maze.width();
                render_cell(
                    render_group,
                    x,
                    y,
                    Color {
                        r: 34,
                        g: 110,
                        b: 110,
                    },
                );
            }
            WilsonStep::Link(cell, next) => {
                for idx in &self.current_walk {
                    let x = idx % self.maze.width();
                    let y = idx / self.maze.width();
                    render_cell(
                        render_group,
                        x,
                        y,
                        Color {
                            r: 34,
                            g: 110,
                            b: 110,
                        },
                    );
                }
                let x = cell % self.maze.width();
                let y = cell / self.maze.width();
                render_cell(
                    render_group,
                    x,
                    y,
                    Color {
                        r: 34,
                        g: 110,
                        b: 110,
                    },
                );
            }
            WilsonStep::Finished => {}
        }
    }

    fn next(&mut self, rng: &mut StdRng) {
        if self.unvisited.is_empty() {
            return;
        }

        let next = match self.steps[self.next] {
            WilsonStep::Empty => {
                let cell = self.unvisited.pop().expect("maze cant be empty");
                self.visited.push(cell);
                WilsonStep::StartWalk
            }
            WilsonStep::StartWalk => {
                let random = self
                    .unvisited
                    .last()
                    .cloned()
                    .expect("maze cant be empty (or len = 1)");
                WilsonStep::RandDir(random)
            }
            WilsonStep::RandDir(cell) => {
                let neighbors = self.maze.neighbors(cell);
                let next = neighbors.iter().choose(rng).unwrap().idx;
                WilsonStep::Walk(cell, next)
            }
            WilsonStep::Walk(cell, next) => {
                if self.current_walk.contains(&next) {
                    self.current_walk.push(cell);
                    WilsonStep::EraseWalk(next)
                } else if self.unvisited.contains(&next) {
                    self.current_walk.push(cell);
                    WilsonStep::RandDir(next)
                } else {
                    WilsonStep::FinishWalk(cell, next)
                }
            }
            WilsonStep::EraseWalk(cell) => {
                let idx = self
                    .current_walk
                    .iter()
                    .position(|x| *x == cell)
                    .expect("cannot find cell in walk");
                let truncated = self.current_walk.drain(idx..);
                let len = truncated.len();
                self.truncated_walk.extend(truncated);
                self.truncated_walk.push(len);
                WilsonStep::RandDir(cell)
            }
            WilsonStep::FinishWalk(cell, next) => WilsonStep::Link(cell, next),
            WilsonStep::Link(from, to) => {
                let unvisited_idx = self.unvisited.iter().position(|x| *x == from).unwrap();
                let link = Link {
                    unvisited_idx,
                    to,
                    from,
                };
                let dir = self.maze.from_a_to_b(link.from, link.to).unwrap();
                self.maze.link(link.from, dir);
                self.visited.push(link.from);
                self.unvisited.swap_remove(link.unvisited_idx);
                self.links.push(link);
                if self.current_walk.is_empty() {
                    if self.unvisited.is_empty() {
                        WilsonStep::Finished
                    } else {
                        WilsonStep::StartWalk
                    }
                } else {
                    WilsonStep::Link(self.current_walk.pop().unwrap(), from)
                }
            }
            WilsonStep::Finished => {
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
            WilsonStep::Empty => {
                let last = self.visited.pop().unwrap();
                self.unvisited.push(last);
            }
            WilsonStep::StartWalk => {}
            WilsonStep::RandDir(_) => {}
            WilsonStep::Walk(cell, _) => {
                let last_walk = self.current_walk.pop().unwrap();
                assert_eq!(last_walk, cell);
            }
            WilsonStep::EraseWalk(_) => {
                let len = self.truncated_walk.pop().unwrap();
                let offset = self.truncated_walk.len() - len;
                let truncated = self.truncated_walk.drain(offset..);
                self.current_walk.extend(truncated);
            }
            WilsonStep::FinishWalk(_, _) => {}
            WilsonStep::Link(_, _) => {
                let link = self.links.pop().unwrap();
                let dir = self.maze.from_a_to_b(link.from, link.to).unwrap();
                self.maze.unlink(link.from, dir);
                self.visited.pop().unwrap();
                let last = self.unvisited.len();
                self.unvisited.push(link.from);
                self.unvisited.swap(last, link.unvisited_idx);
                self.current_walk.push(link.from);
            }
            WilsonStep::Finished => {}
        }
    }

    fn finished(&self) -> bool {
        self.steps[self.next] == WilsonStep::Finished
    }

    fn steps_count(&self) -> usize {
        self.steps.len()
    }

    fn maze(&self) -> &Maze {
        &self.maze
    }

    fn next_step(&self) -> usize {
        self.next
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Link {
    pub unvisited_idx: usize,
    pub from: usize,
    pub to: usize,
}
