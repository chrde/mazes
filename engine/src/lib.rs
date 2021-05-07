use crate::game_host_api::GameHostApi;
use crate::host_api::*;
use crate::plugin::Plugin;
use sdl2::pixels::Color;
use sdl2::{event::Event, rect::Rect};
use sdl2::{keyboard::Keycode, video::GLProfile};
use std::{sync::mpsc::Receiver, time::Duration};

mod game_host_api;
#[path = "../../src/host_api.rs"]
mod host_api;
mod plugin;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

const WIDTH: u32 = 1024;

pub fn run(reloader: Receiver<()>) -> Result<(), String> {
    let font = "/nix/store/ah9gyp7rxak9ig2q829myn6172jn302f-hack-font-3.003/share/fonts/hack/Hack-Regular.ttf";
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(3, 3);
    let ttf_context = sdl2::ttf::init().unwrap();

    let font = ttf_context.load_font(font, 11).unwrap();
    let (font_w, font_h) = font.size_of_char('a').unwrap();

    let window = video_subsystem
        .window("rust-sdl2 demo: Video", WIDTH, WIDTH)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);
    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (3, 3));

    let mut host_api = GameHostApi::new(font_w, font_h);

    let mut plugin = Plugin::new("./target/release/libgenerator.so".to_string()).unwrap();
    let mut game_api = plugin.api().unwrap();
    let state = (game_api.init)(&mut host_api);

    let mut canvas = window
        .into_canvas()
        .index(find_sdl_gl_driver().unwrap())
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();
    let mut texture = texture_creator
        .create_texture_target(None, 1024, 1024)
        .unwrap();

    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    // let border_height = (tile_height as f32 * 0.1) as u32;

    'running: loop {
        if reloader.try_recv().is_ok() {
            println!("===== Reloading =====");
            std::mem::drop(game_api);
            plugin.reload().unwrap();
            game_api = plugin.api().unwrap();
        }

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown {
                    keycode: Some(Keycode::Return),
                    ..
                } => {
                    println!("===== Restarting =====");
                    (game_api.restart)(state, &mut host_api);
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        (game_api.update)(state, &mut host_api);

        // let mouse_state = event_pump.mouse_state();
        // let selected_cell = if mouse_state.left() {
        //     let (x, y) = (mouse_state.x(), mouse_state.y());
        //     let tile_x = x / tile_width;
        //     let tile_y = y / tile_height;
        //     Some((tile_y * grid.width() as i32 + tile_x) as usize)
        // } else {
        //     None
        // };

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        canvas
            .with_texture_canvas(&mut texture, |canvas| {
                canvas.set_draw_color(Color::RGB(0, 0, 0));
                canvas.clear();
                for render_command in host_api.render_group().drain() {
                    match render_command {
                        RenderCommand::FillRectangle {
                            x,
                            y,
                            width,
                            height,
                            color,
                        } => {
                            let rect = Rect::new(x, y, width, height);
                            canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
                            canvas.fill_rect(rect).unwrap();
                        }
                        RenderCommand::DrawRectangle {
                            x,
                            y,
                            width,
                            height,
                            color,
                        } => {
                            let rect = Rect::new(x, y, width, height);
                            canvas.set_draw_color(Color::RGB(color.r, color.g, color.b));
                            canvas.draw_rect(rect).unwrap();
                        }
                        RenderCommand::Text {
                            x,
                            y,
                            width,
                            height,
                            text,
                        } => {
                            let rect = Rect::new(x, y, width, height);
                            let surface =
                                font.render(&text).solid(Color::RGB(255, 255, 255)).unwrap();
                            let text = texture_creator
                                .create_texture_from_surface(surface)
                                .unwrap();
                            canvas.copy(&text, None, rect).unwrap();
                        }
                    }
                }
            })
            .unwrap();
        canvas.copy(&texture, None, None).unwrap();
        canvas.present();
    }
    ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    // The rest of the game loop goes here...

    Ok(())
}
