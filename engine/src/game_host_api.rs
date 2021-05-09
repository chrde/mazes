use crate::host_api::*;

#[derive(Clone, Debug)]
pub struct GameHostApi {
    game_render_group: RenderGroup,
    font_w: f32,
    font_h: f32,
}

impl GameHostApi {
    pub fn new(font_w: f32, font_h: f32) -> Self {
        Self {
            game_render_group: Default::default(),
            font_w,
            font_h,
        }
    }
}

impl HostApi for GameHostApi {
    fn render_group(&mut self) -> &mut RenderGroup {
        &mut self.game_render_group
    }

    fn font_w(&self) -> f32 {
        self.font_w
    }

    fn font_h(&self) -> f32 {
        self.font_h
    }
}
