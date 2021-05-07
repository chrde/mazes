use rand::prelude::ThreadRng;

use crate::host_api::*;

#[derive(Clone, Debug)]
pub struct GameHostApi {
    game_render_group: RenderGroup,
    rng: ThreadRng,
    font_w: u32,
    font_h: u32,
}

impl GameHostApi {
    pub fn new(font_w: u32, font_h: u32) -> Self { Self { game_render_group: Default::default(), rng: rand::thread_rng(), font_w, font_h, } }
}

impl HostApi for GameHostApi {
    fn render_group(&mut self) -> &mut RenderGroup {
        &mut self.game_render_group
    }

    fn rng(&mut self) -> &mut ThreadRng {
        &mut self.rng
    }

    fn font_w(&self) -> u32 {
        self.font_w
    }

    fn font_h(&self) -> u32 {
        self.font_h
    }
}
