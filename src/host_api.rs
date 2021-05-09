#![allow(dead_code)]
use rand::prelude::ThreadRng;

pub trait HostApi {
    fn hello(&self) -> &str {
        "hello world"
    }
    fn render_group(&mut self) -> &mut RenderGroup;
    fn rng(&mut self) -> &mut ThreadRng;
    fn font_w(&self) -> f32;
    fn font_h(&self) -> f32;
}

#[derive(Clone, Debug, Default)]
pub struct RenderGroup {
    commands: Vec<RenderCommand>,
}

impl RenderGroup {
    pub fn push(&mut self, command: RenderCommand) {
        self.commands.push(command);
    }

    pub fn drain(&mut self) -> impl Iterator<Item = RenderCommand> + '_ {
        self.commands.drain(..)
    }

    pub fn len(&self) -> usize {
        self.commands.len()
    }
}

#[derive(Clone, Debug)]
pub enum RenderCommand {
    FillRectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    },
    DrawRectangle {
        x: f32,
        y: f32,
        width: f32,
        height: f32,
        color: Color,
    },
    Text {
        x: f32,
        y: f32,
        text: String,
    },
}

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn gradient_gray(perc: f64) -> Self {
        assert!(perc <= 1.1);
        assert!(perc >= 0.0);
        let color = (255.0 * (1.0 - perc)) as u8;
        Self {
            r: color,
            g: color,
            b: color,
        }
    }
}
