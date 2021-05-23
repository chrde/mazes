use crate::{maze::Maze, render::render_borders};
use host_api::{Color, RenderGroup};
use rand::prelude::StdRng;

mod binary_tree;
mod sidewinder;
mod wilson;

pub use self::binary_tree::BinaryTreeGen;
pub use self::sidewinder::SidewinderGen;
pub use self::wilson::WilsonGen;

pub trait MazeGenerator {
    fn render(&mut self, render_group: &mut RenderGroup, border_color: Color) {
        for y in 0..self.maze().height() {
            for x in 0..self.maze().width() {
                render_borders(render_group, x, y, self.maze(), border_color);
            }
        }
    }
    fn next(&mut self, rng: &mut StdRng);
    fn prev(&mut self);

    /// are there more steps?
    fn finished(&self) -> bool;

    /// has the maze been generated?
    fn completed(&self) -> bool;

    fn steps_count(&self) -> usize;
    fn next_step(&self) -> usize;
    fn maze(&self) -> &Maze;

    fn finish(&mut self, rng: &mut StdRng) {
        while !self.finished() {
            self.next(rng);
        }
    }

    fn goto_step(&mut self, rng: &mut StdRng, step: usize) {
        if step < self.next_step() {
            while step < self.next_step() {
                self.prev();
            }
        } else if step > self.next_step() {
            while step > self.next_step() && !self.finished() {
                self.next(rng);
            }
        }
    }
}

#[derive(Eq, PartialEq)]
pub enum Generator {
    BinaryTree,
    Sidewind,
    Wilson,
}
