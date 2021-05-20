use crate::{common::*, gen::MazeGenerator};
use crate::{common::*, render::DARK_RED, render_borders, render_cell, Color, RenderGroup, RED};
use rand::prelude::StdRng;
use rand::Rng;

// pub fn generate(rng: &mut StdRng, width: usize, height: usize) -> (Maze, Vec<Step>) {
//     let mut maze = Maze::new(width, height);
//     let mut current_group = 1usize;
//     let mut steps = vec![];
//     let mut last_direction = None;
//     for cell in 0..maze.len() {
//         let east = maze.neighbor_at(cell, Neighbor::East);
//         let north = maze.neighbor_at(cell, Neighbor::North);
//         let finish_group = match (east, north) {
//             (Some(_), Some(_)) => rng.gen_range(0..2) == 0,
//             (Some(_), None) => false,
//             (None, Some(_)) => true,
//             (None, None) => true,
//         };
//         if finish_group {
//             if north.is_some() {
//                 let rand = rng.gen_range(0..current_group);
//                 maze.link(cell - rand, Neighbor::North);
//                 steps.push(Step::Direction {
//                     old: last_direction,
//                     new: Neighbor::North,
//                 });
//                 steps.push(Step::Link(cell - rand, Neighbor::North));
//                 last_direction = Some(Neighbor::North);
//             }
//             current_group = 1;
//         } else {
//             current_group += 1;
//             let new = Neighbor::East;
//             maze.link(cell, new);
//             steps.push(Step::Direction {
//                 old: last_direction,
//                 new,
//             });
//             steps.push(Step::Link(cell, new));
//             last_direction = Some(new);
//         }
//     }
//     (maze, steps)
// }

#[derive(Copy, Clone, Debug)]
enum Step {
    Empty,
    Direction(usize),
    Link(usize, Neighbor1),
    Finished,
}

pub struct SidewindGen {
    maze: Maze,
    next: usize,
    steps: Vec<Step>,
}

impl SidewindGen {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            maze: Maze::new(width, height),
            next: 0,
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
                let neighbors: Vec<_> = [Neighbor::North, Neighbor::East]
                    .iter()
                    .cloned()
                    .filter_map(|n| self.maze.neighbor_at(cell, n))
                    .collect();
                if neighbors.is_empty() {
                    Step::Direction(cell + 1)
                } else {
                    let choice = neighbors[rng.gen_range(0..neighbors.len())];
                    Step::Link(cell, choice)
                }
            }
            Step::Link(cell, next) => {
                self.maze.link(cell, next.dir);
                if cell == self.maze.len() - 1 {
                    Step::Finished
                } else {
                    Step::Direction(cell + 1)
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
            Step::Empty => {}
            Step::Direction(_) => {}
            Step::Link(cell, next) => self.maze.unlink(cell, next.dir),
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
