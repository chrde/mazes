fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (rx, _watcher) = reloader::run()?;
    engine::run(rx)?;
    Ok(())
}
