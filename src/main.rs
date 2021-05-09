use macroquad::prelude::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Window Conf".to_owned(),
        window_width: engine::WIDTH,
        window_height: engine::WIDTH,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let (rx, _watcher) = reloader::run().unwrap();
    engine::run(rx).await.unwrap();
}
