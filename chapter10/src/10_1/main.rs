use notify::{watcher, RecursiveMode, Watcher};
use std::time::Duration;
use std::{path::Path, sync::mpsc::channel};

fn main() {
    let mut counter = 0u32;
    let monitoring_target = Path::new(".");
    // Create a channel to receive the events.
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10))
        .expect("I'm not thinking about the case where the watcher creation fails.");

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher
        .watch(monitoring_target, RecursiveMode::Recursive)
        .expect("std::env::current_dir shouldn't fail");

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
