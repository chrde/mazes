use crate::{common::Maze, Color, RenderGroup};
use rand::prelude::StdRng;

pub trait MazeGenerator {
    fn render(&mut self, render_group: &mut RenderGroup, border_color: Color);
    fn next(&mut self, rng: &mut StdRng);
    fn prev(&mut self);
    fn finished(&self) -> bool;
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
