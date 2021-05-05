use std::env;
use std::fs::File;
use std::io;

fn main() -> io::Result<()> {
    let args: Vec<_> = env::args().collect();

    let src = &args[1];
    let dest = &args[2];

    let mut file = File::open(src)?;
    let mut new_file = File::create(dest)?;

    io::copy(&mut file, &mut new_file)?;

    Ok(())
}
