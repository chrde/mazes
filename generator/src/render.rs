use crate::host_api::*;
use crate::Maze;

pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
};

pub fn render_cell(render_group: &mut RenderGroup, x: i32, y: i32, maze: &Maze) {
    let tile_width = 100;
    let tile_height = 100;
    let border_height = 3;
    let border_width = 3;
    let idx = y as usize * maze.width() + x as usize;
    let links = maze.cells()[idx].links();
    if !links.north {
        let command = RenderCommand::FillRectangle {
            x: (x * tile_width) as f32,
            y: (y * tile_height) as f32,
            width: tile_width as f32,
            height: border_height as f32,
            color: YELLOW,
        };
        render_group.push(command);
    }
    if !links.south {
        let command = RenderCommand::FillRectangle {
            x: (x * tile_width) as f32,
            y: ((y + 1) * tile_height - border_height as i32) as f32,
            width: tile_width as f32,
            height: border_height as f32,
            color: YELLOW,
        };
        render_group.push(command);
    }
    if !links.east {
        let command = RenderCommand::FillRectangle {
            x: ((x + 1) * tile_width - border_width as i32) as f32,
            y: (y * tile_height) as f32,
            width: border_width as f32,
            height: tile_height as f32,
            color: YELLOW,
        };
        render_group.push(command);
    }
    if !links.west {
        let command = RenderCommand::FillRectangle {
            x: (x * tile_width) as f32,
            y: (y * tile_height) as f32,
            width: border_width as f32,
            height: tile_height as f32,
            color: YELLOW,
        };
        render_group.push(command);
    }
}
