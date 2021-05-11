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
        debug_borders_color: [117, 140, 140],
        debug_autoplay: false,
        reload_requested: false,
        debug_step: steps.len() - 1,
        debug_maze_width: maze_width,
        debug_maze_height: maze_height,
        debug_show_distances: false,
    };
    let generation = Generation::finished(steps);
    let distances = dijkstra::flood(maze.middle_cell(), &maze);
    let path = dijkstra::longest_path(&maze);
    let game = GameState {
        rng,
        generation,
        debug,
        maze_width,
        maze_height,
        maze,
        distances,
        path,
        generator,
        camera_zoom: 2.5,
        camera_x: 0.0,
        camera_y: 0.0,
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

    pub fn len(&self) -> usize {
        self.steps.len()
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
        ui.horizontal(|ui| {
            ui.radio_value(&mut state.generator, Generator::Sidewind, "Sidewind");
            ui.radio_value(&mut state.generator, Generator::BinaryTree, "Binary tree");
        });
        ui.horizontal(|ui| {
            ui.label("Width:");
            ui.add(
                Slider::new(&mut state.debug.debug_maze_width, 1..=90)
                    .clamp_to_range(true)
                    .integer(),
            );
            ui.label("Height: ");
            ui.add(
                Slider::new(&mut state.debug.debug_maze_height, 1..=90)
                    .clamp_to_range(true)
                    .integer(),
            );
        });
        ui.horizontal(|ui| {
            ui.label("Steps:");
            let slider = Slider::new(
                &mut state.debug.debug_step,
                0..=state.generation.steps.len() - 1,
            )
            .clamp_to_range(true);
            ui.add(slider);
            let first = Button::new("⏮").enabled(state.debug.debug_step != 0);
            if ui.add(first).clicked() {
                state.debug.debug_step = 0;
            }
            let prev = Button::new("⏪").enabled(state.debug.debug_step != 0);
            if ui.add(prev).clicked() {
                state.debug.debug_step -= 1;
            }
            if state.debug.debug_autoplay {
                let pause = Button::new("⏸").enabled(state.debug.debug_autoplay);
                if ui.add(pause).clicked() {
                    state.debug.debug_autoplay = false;
                }
            } else {
                let play = Button::new("⏵").enabled(!state.debug.debug_autoplay);
                if ui.add(play).clicked() {
                    state.debug.debug_autoplay = true;
                }
            }
            let next = Button::new("⏩")
                .enabled(state.debug.debug_step != state.generation.steps.len() - 1);
            if ui.add(next).clicked() {
                state.debug.debug_step += 1;
            }
            let last = Button::new("⏭")
                .enabled(state.debug.debug_step != state.generation.steps.len() - 1);
            if ui.add(last).clicked() {
                state.debug.debug_step = state.generation.steps.len() - 1;
            }
        });
        ui.horizontal(|ui| {
            ui.color_edit_button_srgb(&mut state.debug.debug_borders_color);
            ui.checkbox(&mut state.debug.debug_show_distances, "show distances");
        });
        if ui.button("restart").clicked() {
            state.debug.reload_requested = true;
        }
    });
    true
}

pub fn debug_reload_maze(state: &mut GameState, _input: &Input) {
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
        state.distances = dijkstra::flood(state.maze.middle_cell(), &state.maze);
        state.path = dijkstra::longest_path(&state.maze);
        state.debug.reload_requested = false;
        state.debug.debug_step = cmp::min(state.debug.debug_step, state.generation.len() - 1);
    }
    if state.debug.debug_autoplay {
        // if input.elapsed % 0.1 <= 0.1 {
        state.debug.debug_step += 1;
        if state.debug.debug_step == state.generation.len() - 1 {
            state.debug.debug_autoplay = false;
        } else {
            state.debug.debug_step = state.debug.debug_step % (state.generation.len() - 1);
        }
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
    debug_reload_maze(state, input);
    let maze = &mut state.maze;
    if input.mouse_wheel_up {
        state.camera_zoom /= 1.1;
    }
    if input.mouse_wheel_down {
        state.camera_zoom *= 1.1;
    }
    if input.left {
        state.camera_x += 0.1;
    }
    if input.right {
        state.camera_x -= 0.1;
    }
    if input.up {
        state.camera_y -= 0.1;
    }
    if input.down {
        state.camera_y += 0.1;
    }
    host_api.render_group().push(RenderCommand::Camera {
        zoom_y: state.camera_zoom,
        offset_x: state.camera_x,
        offset_y: state.camera_y,
    });
    let max_distance = state.distances.iter().max().cloned().unwrap() as f64;
    {
        for y in 0..state.maze_height {
            for x in 0..state.maze_width {
                let idx = y * state.maze_width + x;
                if state.debug.debug_show_distances {
                    render_cell(
                        host_api.render_group(),
                        x,
                        y,
                        Color::gradient_gray(state.distances[idx] as f64 / max_distance),
                    );
                    // let text = format!("d:{}", state.distances[idx]);
                    // render_cell_text(host_api.render_group(), 0.0, 0.0, text);
                }

                if idx == (state.generation.current_step + 1) / 2 {
                    render_cell(host_api.render_group(), x, y, RED)
                }
                render_borders(
                    host_api.render_group(),
                    x,
                    y,
                    maze,
                    Color {
                        r: state.debug.debug_borders_color[0],
                        g: state.debug.debug_borders_color[1],
                        b: state.debug.debug_borders_color[2],
                    },
                );
            }
        }
    }
    let needs_update = true;
    needs_update
}

#[derive(Eq, PartialEq)]
enum Generator {
    BinaryTree,
    Sidewind,
}

#[derive(Clone, Debug)]
pub struct Debug {
    debug_borders_color: [u8; 3],
    reload_requested: bool,
    debug_step: usize,
    debug_autoplay: bool,
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
    camera_x: f32,
    camera_y: f32,
}
