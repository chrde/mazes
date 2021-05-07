#![allow(improper_ctypes_definitions)]

mod binary_tree;
mod common;
mod dijkstra;
#[path = "../../src/host_api.rs"]
mod host_api;
mod render;
mod sidewinder;

use common::Maze;
use render::render_cell;

use crate::host_api::*;

#[no_mangle]
pub extern "C" fn init(host_api: &mut dyn HostApi) -> *mut GameState {
    // new game
    let maze_width = 10;
    let maze_height = 9;
    let generator = Generator::BinaryTree;
    let maze = match generator {
        Generator::BinaryTree => binary_tree::generate(host_api.rng(), maze_width, maze_height),
        Generator::Sidewind => sidewinder::generate(host_api.rng(), maze_width, maze_height),
    };
    let distances = dijkstra::flood(0, &maze);
    let path = dijkstra::longest_path(&maze);
    let game = GameState {
        maze_width,
        maze_height,
        maze,
        distances,
        path,
        generator,
    };
    Box::into_raw(Box::new(game))
}

#[no_mangle]
pub extern "C" fn restart(state: &mut GameState, host_api: &mut dyn HostApi) {
    // lib reloaded
    state.maze = match state.generator {
        Generator::BinaryTree => {
            binary_tree::generate(host_api.rng(), state.maze_width, state.maze_height)
        }
        Generator::Sidewind => {
            sidewinder::generate(host_api.rng(), state.maze_width, state.maze_height)
        }
    };
    state.distances = dijkstra::flood(state.maze.len() / 2, &state.maze);
    state.path = dijkstra::longest_path(&state.maze);
}

#[no_mangle]
pub extern "C" fn update(state: &mut GameState, host_api: &mut dyn HostApi) -> bool {
    let needs_update = true;
    let maze = &mut state.maze;
    let maze_width = state.maze_width;
    let maze_height = state.maze_height;
    let tile_width = 100;
    let tile_height = 100;
    let border_height = 3;
    let border_width = 3;
    let max_distance = state.distances.iter().max().cloned().unwrap() as f64;
    {
        for y in 0..maze_height as i32 {
            for x in 0..maze_width as i32 {
                let idx = y as usize * maze_width + x as usize;
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
                let command = RenderCommand::FillRectangle {
                    x: x * tile_width,
                    y: y * tile_height,
                    width: tile_width as u32,
                    height: tile_height as u32,
                    color: Color::gradient_gray(state.distances[idx] as f64 / max_distance),
                };
                host_api.render_group().push(command);

                let text = format!("d:{}", state.distances[idx]);
                let command = RenderCommand::Text {
                    x: border_height as i32 * 2 + x * tile_width,
                    y: border_width as i32 * 2 + y * tile_height,
                    width: host_api.font_w() * text.len() as u32,
                    height: host_api.font_h(),
                    text,
                };
                host_api.render_group().push(command);
                render_cell(host_api.render_group(), x, y, maze);

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

enum Generator {
    BinaryTree,
    Sidewind,
}

#[repr(C)]
pub struct GameState {
    maze_width: usize,
    maze_height: usize,
    maze: Maze,
    distances: Vec<usize>,
    path: Vec<usize>,
    generator: Generator,
}
