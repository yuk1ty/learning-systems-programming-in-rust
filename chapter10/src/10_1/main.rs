use notify::Watcher as _;
use notify::{raw_watcher, RecursiveMode};
use std::time::Duration;
use std::{path::Path, sync::mpsc::channel};

fn main() {
    let mut counter = 0u32;
    let monitoring_target = Path::new(".");
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher =
        raw_watcher(tx).expect("the initialization of watcher shouldn't fail on this code.");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(monitoring_target, RecursiveMode::Recursive)
        .expect("std::env::current_dir shouldn't fail on this code");

    while counter < 3 {
        match rx.recv() {
            Ok(event) => {
                counter += 1;
                println!("{:?}", event)
            }
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
