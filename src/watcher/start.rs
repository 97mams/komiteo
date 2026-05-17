use notify::{
    Config,
    RecommendedWatcher,
    RecursiveMode,
    Watcher,
};

use std::path::Path;
use std::sync::mpsc::channel;

pub fn watch(path: &str) -> notify::Result<()> {
    let (tx, rx) = channel();

    let mut watcher = RecommendedWatcher::new(
        move |res| {
            tx.send(res).unwrap();
        },
        Config::default(),
    )?;

    watcher.watch(
        Path::new(path),
        RecursiveMode::Recursive,
    )?;

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                println!("changement detected: {:?}", event);
            }

            _ => {}
        }
    }
}