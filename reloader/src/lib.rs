use notify::{RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::channel;
use std::sync::mpsc::Receiver;
use std::time::Duration;

const LIBGAME: &str = "./target/release/libgenerator.so";

pub fn run() -> Result<(Receiver<()>, RecommendedWatcher), Box<dyn std::error::Error>> {
    let libgame = Path::new(LIBGAME).canonicalize().unwrap();
    let path = libgame.parent().unwrap().to_owned();

    let (reload_tx, reload_rx) = channel();
    let (tx_game, rx_game) = channel();

    let mut watcher: RecommendedWatcher = Watcher::new(reload_tx, Duration::from_millis(250))?;

    std::thread::spawn(move || {
        loop {
            match reload_rx.recv() {
                Ok(event) => {
                    dbg!(&event);
                    if let notify::DebouncedEvent::Write(path) = event {
                        if path.iter().any(|x| libgame.ends_with(x)) {
                            // signal that we need to reload
                            tx_game.send(()).expect("reload channel closed?");
                        }
                    }
                }
                Err(e) => {
                    println!("watch error: {:?}", e);
                    return;
                }
            }
        }
    });

    watcher.watch(&path, RecursiveMode::NonRecursive)?;

    Ok((rx_game, watcher))
}
