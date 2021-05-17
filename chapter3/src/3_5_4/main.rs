use lib::{
    env::temp_file,
    image::png::{PngAnalyzer, PngChunk, PngCreator},
};
use std::{
    env,
    fs::File,
    io::{self, Read, Write},
};

fn embed_text_chunk<R: Read>(orig_png_reader: R, text: &str) -> Vec<u8> {
    let mut creator = PngCreator::default();
    let analyzer = PngAnalyzer::new(orig_png_reader);

    let mut chunks = analyzer.chunks();
    let ihdr_chunk = chunks.next().expect("first chunk must be IHDR");
    let text_chunk = PngChunk::new_text_chunk(text.to_string());

    creator.add_chunk(ihdr_chunk);
    creator.add_chunk(text_chunk);
    for chunk in chunks {
        creator.add_chunk(chunk);
    }
    creator.finalize()
}

fn main() -> io::Result<()> {
    let orig_png_path: String = {
        let mut args = env::args();
        let _ = args.next();
        args.next()
            .expect("pass original PNG file path as command line option")
    };

    let orig_png_file = File::open(orig_png_path)?;
    let new_png_bin = embed_text_chunk(orig_png_file, "ASCII PROGRAMMING++");

    let new_png_path = temp_file();
    let mut new_png_file = File::create(&new_png_path)?;
    new_png_file.write_all(&new_png_bin)?;
    println!(
        r#"PNG file copied (with tEXt embedded) to: "{}""#,
        new_png_path.display()
    );

    Ok(())
}
