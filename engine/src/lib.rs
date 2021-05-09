use crate::game_host_api::GameHostApi;
use crate::host_api::{HostApi, RenderCommand, Input};
use crate::plugin::Plugin;
use macroquad::prelude::*;
use std::sync::mpsc::Receiver;

mod game_host_api;
#[path = "../../src/host_api.rs"]
mod host_api;
mod plugin;

pub const WIDTH: i32 = 1024;

pub async fn run(reloader: Receiver<()>) -> Result<(), String> {
    let font = load_ttf_font("/nix/store/ah9gyp7rxak9ig2q829myn6172jn302f-hack-font-3.003/share/fonts/hack/Hack-Regular.ttf").await;
    let (font_w, font_h) = {
        let dim = measure_text("a", Some(font), 11, 1.0);
        (dim.width, dim.height)
    };
    let mut host_api = GameHostApi::new(font_w, font_h);

    let mut plugin = Plugin::new("./target/release/libgenerator.so".to_string()).unwrap();
    let mut game_api = plugin.api().unwrap();
    let state = (game_api.init)(&mut host_api);

    loop {
        if reloader.try_recv().is_ok() {
            println!("===== Reloading =====");
            std::mem::drop(game_api);
            plugin.reload().unwrap();
            game_api = plugin.api().unwrap();
        }
        clear_background(BLACK);

        if is_key_pressed(KeyCode::Escape) {
            return Ok(());
        }

        let (_, wheel_y) = mouse_wheel();
        let input = Input {
            mouse_wheel_up: wheel_y > 0.0,
            mouse_wheel_down: wheel_y < 0.0,
        };

        egui_macroquad::ui(|egui_ctx| {
            (game_api.dbg_update)(state, &mut host_api, egui_ctx);
        });
        (game_api.update)(state, &mut host_api, &input);

        for render_command in host_api.render_group().drain() {
            match render_command {
                RenderCommand::DrawRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    let color = Color::from_rgba(color.r, color.g, color.b, 255);
                    draw_rectangle_lines(x, y, width, height, 1.0, color)
                }
                RenderCommand::FillRectangle {
                    x,
                    y,
                    width,
                    height,
                    color,
                } => {
                    let color = Color::from_rgba(color.r, color.g, color.b, 255);
                    draw_rectangle(x, y, width, height, color)
                }
                RenderCommand::Text { x, y, text } => {
                    let params = TextParams {
                        font,
                        font_size: 11,
                        font_scale: 1.0,
                        color: WHITE,
                    };
                    draw_text_ex(&text, x, y, params);
                }
                RenderCommand::Zoom { y } => {
                    println!("hallo {}", y);
                    set_camera(&Camera2D {
                        zoom: vec2(y, y * screen_width() / screen_height()),
                        target: vec2(0.5, 0.5),
                        ..Default::default()
                    });
                }
            }
        }

        // set_default_camera();
        egui_macroquad::draw();
        next_frame().await;
    }
}
