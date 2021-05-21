#![allow(improper_ctypes_definitions)]
#![feature(drain_filter)]

mod binary_tree;
mod common;
mod dijkstra;
mod gen;
#[path = "../../src/host_api.rs"]
mod host_api;
mod render;
mod sidewinder;
mod wilson;

use std::cmp;

use binary_tree::BinaryTreeGen;
use common::Maze;
use egui::{color, Button, CtxRef, ScrollArea, Slider};
use gen::MazeGenerator;
use rand::{prelude::StdRng, SeedableRng};
use render::{render_borders, render_cell, RED};
use sidewinder::SidewindGen;
use wilson::WilsonGen;

use crate::host_api::*;

#[no_mangle]
pub extern "C" fn init(_host_api: &mut dyn HostApi) -> *mut GameState {
    // new game
    let maze_width = 3;
    let maze_height = 3;
    let generator = Generator::Sidewind;
    let mut rng = StdRng::seed_from_u64(1234);
    let debug = Debug {
        debug_borders_color: [117, 140, 140],
        debug_autoplay: false,
        reload_requested: false,
        debug_step: 0,
        debug_maze_width: maze_width,
        debug_maze_height: maze_height,
        debug_show_distances: false,
    };
    // let generation = Generation::finished(steps);
    let distances = vec![]; //dijkstra::flood(maze.middle_cell(), &maze);
    let path = vec![]; //dijkstra::longest_path(&maze);
    let game = GameState {
        wilson: Box::new(SidewindGen::new(maze_width, maze_height)),
        rng,
        debug,
        maze_width,
        maze_height,
        distances,
        path,
        generator,
        camera_zoom: 2.5,
        camera_x: 0.0,
        camera_y: 0.0,
    };
    Box::into_raw(Box::new(game))
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
            ui.radio_value(&mut state.generator, Generator::Wilson, "Wilson");
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
                0..=state.wilson.steps_count() - 1,
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

            let no_more =
                state.wilson.finished() && state.debug.debug_step == state.wilson.steps_count() - 1;
            if no_more {
                state.debug.debug_autoplay = false;
                ui.add(Button::new("⏵").enabled(state.debug.debug_autoplay));
            } else if state.debug.debug_autoplay {
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
            let next = Button::new("⏩").enabled(!no_more);
            if ui.add(next).clicked() {
                state.debug.debug_step += 1;
            }
            let last =
                Button::new("⏭").enabled(state.debug.debug_step != state.wilson.steps_count() - 1);
            if ui.add(last).clicked() {
                state.debug.debug_step = state.wilson.steps_count() - 1;
            }
            if !state.wilson.finished() {
                if ui.add(Button::new("finish")).clicked() {
                    state.wilson.finish(&mut state.rng)
                }
            }
        });
        ui.horizontal(|ui| {
            ui.color_edit_button_srgb(&mut state.debug.debug_borders_color);
            ui.checkbox(&mut state.debug.debug_show_distances, "show distances");
        });
        ui.label(format!(
            "step(wilson {}, debug {})",
            state.wilson.next_step(),
            state.debug.debug_step
        ));
        // ui.separator();
        // ui.label(format!("visited: {:?}", state.wilson.visited));
        // ui.label(format!("current_walk: {:?}", state.wilson.current_walk));
        // ui.label(format!("links: {}", state.wilson.links.len()));
        // ui.label(format!("unvisited: {}", state.wilson.unvisited.len()));
        // ui.separator();
        // let mut area = ScrollArea::from_max_height(150.0);
        // area.show(ui, |ui| {
        //     let half = 3;
        //     let current = state.wilson.next_step();
        //     let first = current.saturating_sub(half);
        //     for x in first..current {
        //         ui.label(format!("{}: {:?}", x, state.wilson.steps[x]));
        //     }
        //     ui.colored_label(
        //         color::Color32::YELLOW,
        //         format!("{}: {:?}", current, state.wilson.steps[current]),
        //     );
        //     let last = (current + half) % state.wilson.steps_count();
        //     for x in (current + 1)..last {
        //         ui.label(format!("{}: {:?}", x, state.wilson.steps[x]));
        //     }
        // });
        // ui.separator();
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
        state.debug.reload_requested = false;
        state.wilson = match state.generator {
            Generator::BinaryTree => {
                Box::new(BinaryTreeGen::new(state.maze_width, state.maze_height))
            }
            Generator::Sidewind => Box::new(SidewindGen::new(state.maze_width, state.maze_height)),
            Generator::Wilson => Box::new(WilsonGen::new(
                &mut state.rng,
                state.maze_width,
                state.maze_height,
            )),
        };
        state.debug.debug_step = cmp::min(state.debug.debug_step, state.wilson.steps_count() - 1);
    }
    if state.debug.debug_autoplay {
        state.debug.debug_step = (state.debug.debug_step % state.wilson.steps_count()) + 1;
    }
    if state.debug.debug_step != state.wilson.next_step() {
        state
            .wilson
            .goto_step(&mut state.rng, state.debug.debug_step);
        state.debug.debug_step = state.wilson.next_step();
    }
}

#[no_mangle]
pub extern "C" fn update(state: &mut GameState, host_api: &mut dyn HostApi, input: &Input) -> bool {
    debug_reload_maze(state, input);
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
    let border_color = Color {
        r: state.debug.debug_borders_color[0],
        g: state.debug.debug_borders_color[1],
        b: state.debug.debug_borders_color[2],
    };
    // let max_distance = state.distances.iter().max().cloned().unwrap() as f64;
    state.wilson.render(host_api.render_group(), border_color);
    let needs_update = true;
    needs_update
}

#[derive(Eq, PartialEq)]
enum Generator {
    BinaryTree,
    Sidewind,
    Wilson,
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
    wilson: Box<dyn MazeGenerator>,
    debug: Debug,
    maze_width: usize,
    maze_height: usize,
    distances: Vec<usize>,
    path: Vec<usize>,
    generator: Generator,
    rng: StdRng,
    camera_zoom: f32,
    camera_x: f32,
    camera_y: f32,
}
