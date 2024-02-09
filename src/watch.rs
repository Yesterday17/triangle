use crate::{AppState, Config};
use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use parking_lot::RwLock;
use std::path::Path;
use std::process::exit;
use std::sync::mpsc::channel;
use std::time::Duration;

pub fn watch<P: AsRef<Path>>(path: P, lock: P, data: actix_web::web::Data<RwLock<AppState>>) {
    let (tx, rx) = channel();

    let path = path.as_ref().canonicalize().unwrap();
    let lock_path = lock.as_ref().canonicalize().unwrap();

    std::thread::spawn(move || {
        let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
        watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();
        loop {
            match rx.recv() {
                Ok(event) => match event {
                    DebouncedEvent::Write(_) => {
                        // reload data
                        let mut write = data.write();
                        *write = Config::new(&path, &lock_path).into_state();
                        eprintln!("Config reloaded");
                    }
                    _ => {}
                },
                Err(e) => {
                    eprintln!("watch error: {:?}", e);
                    exit(1);
                }
            }
        }
    });
}
