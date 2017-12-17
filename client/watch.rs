
use std::sync::mpsc;
use std::time::Duration;
use std::path::PathBuf;

use notify::{Watcher, RecursiveMode, watcher};

pub fn dir_watch_prodecure(watch_dirs: Vec<PathBuf>) {

    let (tx, rx) = mpsc::channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();
    for d in watch_dirs {
        watcher.watch(d, RecursiveMode::Recursive).unwrap();
    }

    loop {
        match rx.recv() {
            Ok(event) => println!("{:?}", event),
            Err(e) => println!("watch error: {:?}", e)
        }
    }

}
