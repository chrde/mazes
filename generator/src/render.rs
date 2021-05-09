use crate::host_api::*;
use crate::Maze;

pub const RED: Color = Color {
    r: 179,
    g: 45,
    b: 0,
};

pub const YELLOW: Color = Color {
    r: 255,
    g: 255,
    b: 0,
};

pub fn render_cell(render_group: &mut RenderGroup, x: usize, y: usize, color: Color) {
    let command = RenderCommand::FillRectangle {
        x: x as f32 * TILE_WIDTH,
        y: y as f32 * TILE_HEIGHT,
        width: TILE_WIDTH,
        height: TILE_HEIGHT,
        color,
    };
    render_group.push(command);
}

const TILE_WIDTH: f32 = 0.1;
const TILE_HEIGHT: f32 = 0.1;
const BORDER_WIDTH: f32 = 0.003;
const BORDER_HEIGHT: f32 = 0.003;

// x and y start from table borders
pub fn render_cell_text(render_group: &mut RenderGroup, x: f32, y: f32, text: String) {
    let command = RenderCommand::Text {
        x: x * BORDER_WIDTH,
        y: y * BORDER_HEIGHT,
        text,
    };
    render_group.push(command);
}


pub fn render_borders(render_group: &mut RenderGroup, x: usize, y: usize, maze: &Maze) {
    let idx = y * maze.width() + x;
    let links = maze.cells()[idx].links();
    let cell_x = x as f32 * TILE_WIDTH;
    let cell_y = y as f32 * TILE_HEIGHT;
    if !links.north {
        let command = RenderCommand::FillRectangle {
            x: cell_x,
            y: cell_y,
            width: TILE_WIDTH,
            height: BORDER_HEIGHT,
            color: YELLOW,
        };
        render_group.push(command);
    }
    if !links.south {
        let command = RenderCommand::FillRectangle {
            x: cell_x,
            y: cell_y + TILE_HEIGHT - BORDER_HEIGHT,
            width: TILE_WIDTH,
            height: BORDER_HEIGHT,
            color: YELLOW,
        };
        render_group.push(command);
    }
    if !links.east {
        let command = RenderCommand::FillRectangle {
            x: cell_x + TILE_WIDTH - BORDER_WIDTH,
            y: cell_y,
            width: BORDER_WIDTH,
            height: TILE_HEIGHT,
            color: YELLOW,
        };
        render_group.push(command);
    }
    if !links.west {
        let command = RenderCommand::FillRectangle {
            x: cell_x,
            y: cell_y,
            width: BORDER_WIDTH,
            height: TILE_HEIGHT,
            color: YELLOW,
        };
        render_group.push(command);
    }
}
