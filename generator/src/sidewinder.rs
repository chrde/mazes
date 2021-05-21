use crate::{common::*, gen::MazeGenerator};
use crate::{common::*, render::DARK_RED, render_borders, render_cell, Color, RenderGroup, RED};
use rand::prelude::{SliceRandom, StdRng};
use rand::Rng;

#[derive(Copy, Clone, Debug)]
enum Step {
    Empty,
    Direction(usize),
    WalkEast(usize),
    EraseWalk(usize),
    RandWalk(usize),
    LinkNorth(usize, usize),
    Finished,
}

pub struct SidewindGen {
    maze: Maze,
    next: usize,
    current_walk: Vec<usize>,
    truncated_walk: Vec<usize>,
    steps: Vec<Step>,
}

impl SidewindGen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            maze: Maze::new(width, height),
            next: 0,
            current_walk: vec![],
            truncated_walk: vec![],
            steps: vec![Step::Empty],
        }
    }
}

impl MazeGenerator for SidewindGen {
    fn render(&mut self, render_group: &mut RenderGroup, border_color: Color) {
        for y in 0..self.maze.height() {
            for x in 0..self.maze.width() {
                let idx = y * self.maze.width() + x;
                // if state.debug.debug_show_distances {
                //     render_cell(
                //         host_api.render_group(),
                //         x,
                //         y,
                //         Color::gradient_gray(state.distances[idx] as f64 / max_distance),
                //     );
                //     // let text = format!("d:{}", state.distances[idx]);
                //     // render_cell_text(host_api.render_group(), 0.0, 0.0, text);
                // }

                // if idx == (state.generation.current_step + 1) / 2 {
                //     render_cell(host_api.render_group(), x, y, RED)
                // }
                render_borders(render_group, x, y, &self.maze, border_color);
            }
        }
    }

    fn next(&mut self, rng: &mut StdRng) {
        let next = match self.steps[self.next] {
            Step::Empty => Step::Direction(0),
            Step::Direction(cell) => {
                if cell == self.maze.len() {
                    Step::Finished
                } else {
                    let east = self.maze.neighbor_at(cell, Neighbor::East);
                    let north = self.maze.neighbor_at(cell, Neighbor::North);
                    let finish_walk = match (east, north) {
                        (Some(_), Some(_)) => rng.gen_bool(0.5),
                        (Some(_), None) => false,
                        (None, Some(_)) => true,
                        (None, None) => true,
                    };
                    if finish_walk {
                        if north.is_some() {
                            Step::RandWalk(cell)
                        } else {
                            Step::EraseWalk(cell)
                        }
                    } else {
                        Step::WalkEast(cell)
                    }
                }
            }
            Step::WalkEast(cell) => {
                self.current_walk.push(cell);
                self.maze.link(cell, Neighbor::East);
                Step::Direction(cell + 1)
            }
            Step::EraseWalk(cell) => {
                let len = self.current_walk.len();
                self.truncated_walk.append(&mut self.current_walk);
                self.truncated_walk.push(len);
                Step::Direction(cell + 1)
            }
            Step::RandWalk(cell) => {
                self.current_walk.push(cell);
                let linked = self.current_walk.choose(rng).unwrap();
                Step::LinkNorth(cell, *linked)
            }
            Step::LinkNorth(cell, linked) => {
                self.maze.link(linked, Neighbor::North);
                Step::EraseWalk(cell)
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
            Step::Direction(_) => {}
            Step::WalkEast(cell) => {
                assert_eq!(cell, self.current_walk.pop().unwrap());
                self.maze.unlink(cell, Neighbor::East);
            }
            Step::EraseWalk(cell) => {
                let len = self.truncated_walk.pop().unwrap();
                let offset = self.truncated_walk.len() - len;
                let truncated = self.truncated_walk.drain(offset..);
                self.current_walk.extend(truncated);
            }
            Step::RandWalk(cell) => {
                assert_eq!(cell, self.current_walk.pop().unwrap());
            }
            Step::LinkNorth(cell, linked) => {
                self.maze.unlink(linked, Neighbor::North);
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
}
