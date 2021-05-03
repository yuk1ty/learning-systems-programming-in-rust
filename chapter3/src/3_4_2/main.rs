use std::fs::File;

fn main() {
    let mut src_file = File::open("main.rs").unwrap();

    let mut dest_file = File::create("was_copied.txt").unwrap();

    std::io::copy(&mut src_file, &mut dest_file).unwrap();
}
