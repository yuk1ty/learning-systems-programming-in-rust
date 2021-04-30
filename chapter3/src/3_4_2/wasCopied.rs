use std::fs::File;

fn main() {
    let mut src_file = File::open("main.rs").unwrap();

    let mut dest_file = File::create("wasCopied.rs").unwrap();

    std::io::copy(&mut src_file, &mut dest_file).unwrap();
}