use std::{env, fs::File, io};

use png::PngAnalyzer;

mod png;

fn main() -> io::Result<()> {
    let png_path: String = {
        let mut args = env::args();
        let _ = args.next();
        args.next()
            .expect("pass PNG file path as command line option")
    };

    let png = File::open(png_path)?;
    let analyzer = PngAnalyzer::new(png);
    for chunk in analyzer.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}
