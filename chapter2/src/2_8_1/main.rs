//! Write format string to a file using `write!` macro (Go version uses `Fprintf`).

use std::{
    fmt,
    fs::File,
    io::{self, Read, Write},
};

use lib::env::temp_file;

fn write_fmt<W: fmt::Write>(w: &mut W) {
    writeln!(
        w,
        "{{}}(\"10\"): {}, {{}}(10): {}, {{:.1}}(10.0): {:.1}",
        "10", 10, 10.0
    )
    .expect("wrong format string")
}

fn main() -> io::Result<()> {
    let tmp_path = temp_file();
    {
        let mut contents = String::new();
        write_fmt(&mut contents);
        let mut f = File::create(&tmp_path)?;
        f.write_all(contents.as_bytes())?;
    }
    let written_contents = {
        let mut f = File::open(&tmp_path)?;
        let mut contents = String::new();
        let _ = f.read_to_string(&mut contents)?;
        contents
    };

    println!(
        "Written to '{}' :",
        tmp_path
            .to_str()
            .expect("should ve safely converted into UTF-8")
    );
    println!("{}", written_contents);

    Ok(())
}
