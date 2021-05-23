use super::MazeGenerator;
use crate::{
    maze::Maze,
    render::{DARK_RED, RED},
    render_borders, render_cell,
};
use host_api::{Color, RenderGroup};
use rand::prelude::{IteratorRandom, SliceRandom, StdRng};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Step {
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
    maze: Maze,
    unvisited: Vec<usize>,
    visited: Vec<usize>,
    links: Vec<Link>,
    current_walk: Vec<usize>,
    truncated_walk: Vec<usize>,
    steps: Vec<Step>,
    next: usize,
}

impl WilsonGen {
    pub fn new(rng: &mut StdRng, width: usize, height: usize) -> Self {
        let maze = Maze::new(width, height);
        let mut unvisited: Vec<_> = (0..maze.len()).into_iter().collect();
        unvisited.shuffle(rng);
        let current_walk = vec![];
        let steps = vec![Step::Empty];
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

    fn render_cell(&self, cell: usize, render_group: &mut RenderGroup, color: Color) {
        let x = cell % self.maze.width();
        let y = cell / self.maze.width();
        render_cell(render_group, x, y, color);
    }

    fn render_visited(&self, render_group: &mut RenderGroup) {
        for cell in &self.visited {
            self.render_cell(
                *cell,
                render_group,
                Color {
                    r: 34,
                    g: 70,
                    b: 70,
                },
            )
        }
    }

    fn render_unvisited(&self, render_group: &mut RenderGroup) {
        for cell in &self.unvisited {
            self.render_cell(*cell, render_group, Color { r: 31, g: 0, b: 0 })
        }
    }

    fn render_current_walk(&self, render_group: &mut RenderGroup) {
        for cell in &self.current_walk {
            self.render_cell(
                *cell,
                render_group,
                Color {
                    r: 120,
                    g: 80,
                    b: 105,
                },
            )
        }
    }
}

impl MazeGenerator for WilsonGen {
    fn render(&mut self, render_group: &mut RenderGroup, border_color: Color) {
        match self.steps[self.next] {
            Step::Empty => {}
            Step::StartWalk => {
                self.render_visited(render_group);
                self.render_unvisited(render_group);
            }
            Step::RandDir(cell) => {
                self.render_visited(render_group);
                self.render_unvisited(render_group);
                self.render_current_walk(render_group);
                self.render_cell(cell, render_group, DARK_RED)
            }
            Step::Walk(cell, dir) => {
                self.render_visited(render_group);
                self.render_unvisited(render_group);
                self.render_current_walk(render_group);
                self.render_cell(cell, render_group, DARK_RED);
                self.render_cell(dir, render_group, RED);
            }
            Step::EraseWalk(cell) => {
                self.render_visited(render_group);
                self.render_unvisited(render_group);
                self.render_current_walk(render_group);
                let idx = self
                    .current_walk
                    .iter()
                    .position(|x| *x == cell)
                    .expect("cannot find cell in walk");
                for idx in &self.current_walk[idx..] {
                    self.render_cell(
                        *idx,
                        render_group,
                        Color {
                            r: 210,
                            g: 130,
                            b: 208,
                        },
                    );
                }
            }
            Step::FinishWalk(cell, _) => {
                self.render_visited(render_group);
                self.render_unvisited(render_group);
                for idx in &self.current_walk {
                    self.render_cell(
                        *idx,
                        render_group,
                        Color {
                            r: 34,
                            g: 110,
                            b: 110,
                        },
                    );
                }
                self.render_cell(
                    cell,
                    render_group,
                    Color {
                        r: 34,
                        g: 110,
                        b: 110,
                    },
                );
            }
            Step::Link(cell, _) => {
                self.render_visited(render_group);
                self.render_unvisited(render_group);
                for idx in &self.current_walk {
                    self.render_cell(
                        *idx,
                        render_group,
                        Color {
                            r: 34,
                            g: 110,
                            b: 110,
                        },
                    );
                }
                self.render_cell(
                    cell,
                    render_group,
                    Color {
                        r: 34,
                        g: 110,
                        b: 110,
                    },
                );
            }
            Step::Finished => {}
        }
        for y in 0..self.maze.height() {
            for x in 0..self.maze.width() {
                render_borders(render_group, x, y, &self.maze, border_color);
            }
        }
    }

    fn next(&mut self, rng: &mut StdRng) {
        if self.unvisited.is_empty() {
            return;
        }

        let next = match self.steps[self.next] {
            Step::Empty => {
                let cell = self.unvisited.pop().expect("maze cant be empty");
                self.visited.push(cell);
                Step::StartWalk
            }
            Step::StartWalk => {
                let random = self
                    .unvisited
                    .last()
                    .cloned()
                    .expect("maze cant be empty (or len = 1)");
                Step::RandDir(random)
            }
            Step::RandDir(cell) => {
                let neighbors = self.maze.neighbors(cell);
                let next = neighbors.iter().choose(rng).unwrap().idx;
                Step::Walk(cell, next)
            }
            Step::Walk(cell, next) => {
                if self.current_walk.contains(&next) {
                    self.current_walk.push(cell);
                    Step::EraseWalk(next)
                } else if self.unvisited.contains(&next) {
                    self.current_walk.push(cell);
                    Step::RandDir(next)
                } else {
                    Step::FinishWalk(cell, next)
                }
            }
            Step::EraseWalk(cell) => {
                let idx = self
                    .current_walk
                    .iter()
                    .position(|x| *x == cell)
                    .expect("cannot find cell in walk");
                let truncated = self.current_walk.drain(idx..);
                let len = truncated.len();
                self.truncated_walk.extend(truncated);
                self.truncated_walk.push(len);
                Step::RandDir(cell)
            }
            Step::FinishWalk(cell, next) => Step::Link(cell, next),
            Step::Link(from, to) => {
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
                        Step::Finished
                    } else {
                        Step::StartWalk
                    }
                } else {
                    Step::Link(self.current_walk.pop().unwrap(), from)
                }
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
            Step::Empty => {
                let last = self.visited.pop().unwrap();
                self.unvisited.push(last);
            }
            Step::StartWalk => {}
            Step::RandDir(_) => {}
            Step::Walk(cell, _) => {
                let last_walk = self.current_walk.pop().unwrap();
                assert_eq!(last_walk, cell);
            }
            Step::EraseWalk(_) => {
                let len = self.truncated_walk.pop().unwrap();
                let offset = self.truncated_walk.len() - len;
                let truncated = self.truncated_walk.drain(offset..);
                self.current_walk.extend(truncated);
            }
            Step::FinishWalk(_, _) => {}
            Step::Link(_, _) => {
                let link = self.links.pop().unwrap();
                let dir = self.maze.from_a_to_b(link.from, link.to).unwrap();
                self.maze.unlink(link.from, dir);
                self.visited.pop().unwrap();
                let last = self.unvisited.len();
                self.unvisited.push(link.from);
                self.unvisited.swap(last, link.unvisited_idx);
                self.current_walk.push(link.from);
            }
            Step::Finished => {}
        }
    }

    fn finished(&self) -> bool {
        self.steps[self.next] == Step::Finished
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

    fn completed(&self) -> bool {
        self.steps
            .last()
            .map_or(false, |s| matches!(s, Step::Finished))
    }
}

#[derive(Clone, Eq, PartialEq)]
pub struct Link {
    pub unvisited_idx: usize,
    pub from: usize,
    pub to: usize,
}
