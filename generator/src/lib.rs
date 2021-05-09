#![allow(improper_ctypes_definitions)]

mod binary_tree;
mod common;
mod dijkstra;
#[path = "../../src/host_api.rs"]
mod host_api;
mod render;
mod sidewinder;

use std::cmp;

use common::{Maze, Neighbor};
use egui::{Button, CtxRef, Slider};
use rand::{prelude::StdRng, SeedableRng};
use render::{render_borders, render_cell, render_cell_text, RED};

use crate::host_api::*;

#[no_mangle]
pub extern "C" fn init(_host_api: &mut dyn HostApi) -> *mut GameState {
    // new game
    let maze_width = 10;
    let maze_height = 10;
    let generator = Generator::BinaryTree;
    let mut rng = StdRng::from_entropy();
    let (maze, steps) = match generator {
        Generator::BinaryTree => binary_tree::generate(&mut rng, maze_width, maze_height),
        Generator::Sidewind => sidewinder::generate(&mut rng, maze_width, maze_height),
    };
    let debug = Debug {
        reload_requested: false,
        debug_step: steps.len() - 1,
        debug_maze_width: maze_width,
        debug_maze_height: maze_height,
        debug_show_distances: false,
    };
    let generation = Generation::finished(steps);
    let distances = dijkstra::flood(0, &maze);
    let path = dijkstra::longest_path(&maze);
    let game = GameState {
        camera_zoom: 1.0,
        rng,
        generation,
        debug,
        maze_width,
        maze_height,
        maze,
        distances,
        path,
        generator,
    };
    Box::into_raw(Box::new(game))
}

pub struct Generation {
    current_step: usize,
    // current_direction: Option<Neighbor>,
    steps: Vec<Step>,
}

impl Generation {
    pub fn finished(steps: Vec<Step>) -> Self {
        Self {
            current_step: steps.len() - 1,
            steps,
            // current_direction: None,
        }
    }

    pub fn goto_step(&mut self, maze: &mut Maze, step: usize) {
        if step < self.current_step {
            while step < self.current_step {
                self.undo(maze);
            }
        } else if step > self.current_step {
            while step > self.current_step {
                self.redo(maze);
            }
        }
    }

    pub fn undo(&mut self, maze: &mut Maze) {
        match self.steps[self.current_step] {
            Step::Direction { .. } => {
                // self.current_direction = old;
            }
            Step::Link(cell, neighbor) => {
                maze.unlink(cell, neighbor);
            }
        }
        self.current_step -= 1;
    }

    pub fn redo(&mut self, maze: &mut Maze) {
        match self.steps[self.current_step + 1] {
            Step::Direction { .. } => {
                // self.current_direction = Some(new);
            }
            Step::Link(cell, neighbor) => {
                maze.link(cell, neighbor);
            }
        }
        self.current_step += 1;
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Step {
    Direction {
        old: Option<Neighbor>,
        new: Neighbor,
    },
    Link(usize, Neighbor),
}

#[no_mangle]
pub extern "C" fn dbg_update(
    state: &mut GameState,
    _host_api: &mut dyn HostApi,
    egui_ctx: &CtxRef,
) -> bool {
    egui::Window::new("debug").show(egui_ctx, |ui| {
        // ui.horizontal(|ui| {
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.generator, Generator::Sidewind, "Sidewind");
            ui.radio_value(&mut state.generator, Generator::BinaryTree, "Binary tree");
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(
                Slider::new(&mut state.debug.debug_maze_width, 0..=10)
                    .clamp_to_range(true)
                    .integer(),
            );
            ui.label("Height: ");
            ui.add(
                Slider::new(&mut state.debug.debug_maze_height, 0..=10)
                    .clamp_to_range(true)
                    .integer(),
            );
        });
        // });
        ui.horizontal(|ui| {
            ui.label("Steps:");
            let slider = Slider::new(
                &mut state.debug.debug_step,
                0..=state.generation.steps.len() - 1,
            )
            .clamp_to_range(true);
            ui.add(slider);
            let first = Button::new("⏪").enabled(state.debug.debug_step != 0);
            if ui.add(first).clicked() {
                state.debug.debug_step = 0;
            }
            let prev = Button::new("⏴").enabled(state.debug.debug_step != 0);
            if ui.add(prev).clicked() {
                state.debug.debug_step -= 1;
            }
            let next = Button::new("⏵")
                .enabled(state.debug.debug_step != state.generation.steps.len() - 1);
            if ui.add(next).clicked() {
                state.debug.debug_step += 1;
            }
            let last = Button::new("⏩")
                .enabled(state.debug.debug_step != state.generation.steps.len() - 1);
            if ui.add(last).clicked() {
                state.debug.debug_step = state.generation.steps.len() - 1;
            }
        });
        ui.checkbox(&mut state.debug.debug_show_distances, "show distances");
        if ui.button("restart").clicked() {
            state.debug.reload_requested = true;
        }
    });
    true
}

pub fn debug_reload_maze(state: &mut GameState) {
    if state.debug.reload_requested {
        state.maze_width = state.debug.debug_maze_width;
        state.maze_height = state.debug.debug_maze_height;
        let (maze, steps) = match state.generator {
            Generator::BinaryTree => {
                binary_tree::generate(&mut state.rng, state.maze_width, state.maze_height)
            }
            Generator::Sidewind => {
                sidewinder::generate(&mut state.rng, state.maze_width, state.maze_height)
            }
        };
        state.maze = maze;
        state.generation = Generation::finished(steps);
        state.distances = dijkstra::flood(state.maze.len() / 2, &state.maze);
        state.path = dijkstra::longest_path(&state.maze);
        state.debug.reload_requested = false;
        state.debug.debug_step = cmp::min(state.debug.debug_step, state.generation.steps.len() - 1);
    }
    if state.debug.debug_step != state.generation.current_step {
        state
            .generation
            .goto_step(&mut state.maze, state.debug.debug_step);
        state.debug.debug_step = state.generation.current_step;
    }
}

#[no_mangle]
pub extern "C" fn update(state: &mut GameState, host_api: &mut dyn HostApi, input: &Input) -> bool {
    debug_reload_maze(state);
    let needs_update = true;
    let maze = &mut state.maze;
    let max_distance = state.distances.iter().max().cloned().unwrap() as f64;
    if input.mouse_wheel_up {
        state.camera_zoom *= 1.1;
    }
    if input.mouse_wheel_down {
        state.camera_zoom /= 1.1;
    }
    host_api.render_group().push(RenderCommand::Zoom {
        y: state.camera_zoom,
    });
    {
        for y in 0..state.maze_height {
            for x in 0..state.maze_width {
                let idx = y * state.maze_width + x;
                // if state.path.contains(&idx) {
                //     let command = RenderCommand::FillRectangle {
                //         x: x * tile_width,
                //         y: y * tile_height,
                //         width: tile_width as u32,
                //         height: tile_height as u32,
                //         color: GRAY,
                //     };
                //     host_api.render_group().push(command);
                // }
                if state.debug.debug_show_distances {
                    render_cell(
                        host_api.render_group(),
                        x,
                        y,
                        Color::gradient_gray(state.distances[idx] as f64 / max_distance),
                    );
                    let text = format!("d:{}", state.distances[idx]);
                    render_cell_text(host_api.render_group(), 0.0, 0.0, text);
                    // let command = RenderCommand::Text {
                    //     x: (border_height as i32 * 2 + x * tile_width) as f32,
                    //     y: (border_width as i32 * 2 + y * tile_height) as f32,
                    //     text,
                    // };
                    // host_api.render_group().push(command);
                }

                if idx == (state.generation.current_step + 1) / 2 {
                    render_cell(host_api.render_group(), x, y, RED)
                }
                render_borders(host_api.render_group(), x, y, maze);

                // if selected_cell.map_or(false, |x| x == idx) {
                //     canvas.set_draw_color(Color::RGB(255, 0, 255));
                //     canvas.draw_rect(rect).unwrap();
                //     let surface = font
                //         .render(&format!("{}", idx))
                //         .solid(Color::RGB(255, 255, 255))
                //         .unwrap();
                //     let text = texture_creator
                //         .create_texture_from_surface(surface)
                //         .unwrap();
                //     canvas.copy(&text, None, rect);
                // }
            }
        }
    }
    needs_update
}

#[derive(Eq, PartialEq)]
enum Generator {
    BinaryTree,
    Sidewind,
}

#[derive(Clone, Debug)]
pub struct Debug {
    reload_requested: bool,
    debug_step: usize,
    debug_show_distances: bool,
    debug_maze_width: usize,
    debug_maze_height: usize,
}

#[repr(C)]
pub struct GameState {
    debug: Debug,
    maze_width: usize,
    maze_height: usize,
    maze: Maze,
    distances: Vec<usize>,
    path: Vec<usize>,
    generator: Generator,
    generation: Generation,
    rng: StdRng,
    camera_zoom: f32,
}
