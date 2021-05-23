#![allow(improper_ctypes_definitions)]
#![feature(drain_filter)]

use dbg::debug_reload_maze;
use generator::*;
use host_api::{Color, HostApi, Input, RenderCommand};
use rand::{prelude::StdRng, SeedableRng};
use render::{render_borders, render_cell};

mod dbg;
mod dijkstra;
mod generator;
mod maze;
mod render;

#[no_mangle]
pub extern "C" fn init(_host_api: &mut dyn HostApi) -> *mut GameState {
    // new game
    let maze_width = 15;
    let maze_height = 15;
    let generator = Generator::Sidewind;
    let rng = StdRng::seed_from_u64(1234);
    let debug = Debug {
        debug_borders_color: [117, 140, 140],
        debug_autoplay: false,
        reload_requested: false,
        debug_step: 0,
        debug_maze_width: maze_width,
        debug_maze_height: maze_height,
    };
    let distances = vec![];
    let longest_path = vec![];
    let game = GameState {
        wilson: Box::new(SidewinderGen::new(maze_width, maze_height)),
        rng,
        debug,
        maze_width,
        maze_height,
        distances,
        longest_path,
        generator,
        camera_zoom: 2.5,
        camera_x: 0.0,
        camera_y: 0.0,
        overlay: None,
    };
    Box::into_raw(Box::new(game))
}

#[no_mangle]
pub extern "C" fn update(state: &mut GameState, host_api: &mut dyn HostApi, input: &Input) -> bool {
    debug_reload_maze(state, input);
    if state.wilson.completed() && state.distances.is_empty() {
        state.distances = dijkstra::flood(state.wilson.maze().middle_cell(), &state.wilson.maze());
        dbg!(&state.distances);
    }
    if state.wilson.completed() && state.longest_path.is_empty() {
        state.longest_path = dijkstra::longest_path(state.wilson.maze());
    }
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

    let maze = state.wilson.maze();
    match state.overlay {
        Some(Overlay::Distances) => {
            let max_distance = state.distances.iter().max().cloned().unwrap() as f64;
            for (cell, distance) in state.distances.iter().cloned().enumerate() {
                let x = cell % maze.width();
                let y = cell / maze.width();
                render_cell(
                    host_api.render_group(),
                    x,
                    y,
                    Color::gradient_gray(distance as f64 / max_distance),
                );
                // let text = format!("d:{}", state.distances[idx]);
                // render_cell_text(host_api.render_group(), 0.0, 0.0, text);

                // if idx == (state.generation.current_step + 1) / 2 {
                //     render_cell(host_api.render_group(), x, y, RED)
                // }
            }
        }
        Some(Overlay::LongestPath) => {
            for cell in &state.longest_path {
                let x = cell % maze.width();
                let y = cell / maze.width();
                render_cell(host_api.render_group(), x, y, Color::gradient_gray(0.0));
            }
        }
        None => {}
    }
    state.wilson.render(host_api.render_group(), border_color);
    let needs_update = true;
    needs_update
}

#[derive(Clone, Debug)]
pub struct Debug {
    debug_borders_color: [u8; 3],
    reload_requested: bool,
    debug_step: usize,
    debug_autoplay: bool,
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
    longest_path: Vec<usize>,
    generator: Generator,
    overlay: Option<Overlay>,
    rng: StdRng,
    camera_zoom: f32,
    camera_x: f32,
    camera_y: f32,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Overlay {
    Distances,
    LongestPath,
}
