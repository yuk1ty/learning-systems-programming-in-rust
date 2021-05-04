//! Write format string to a file using `write!` macro (Go version uses `Fprintf`).

use std::{
    env::temp_dir,
    fmt,
    fs::File,
    io::{self, Read, Write},
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};

fn mktemp() -> PathBuf {
    let mut tempdir = temp_dir();

    let tempfile = {
        let now = SystemTime::now();
        let unixtime = now
            .duration_since(UNIX_EPOCH)
            .expect("system clock maybe corrupt");
        format!("{}-{:09}", unixtime.as_secs(), unixtime.subsec_nanos())
    };

    tempdir.push(tempfile);
    tempdir
}

fn write_fmt<W: fmt::Write>(w: &mut W) {
    write!(
        w,
        "{{}}(\"10\"): {}, {{}}(10): {}, {{:.1}}(10.0): {:.1}\n",
        "10", 10, 10.0
    )
    .expect("wrong format string")
}

fn main() -> io::Result<()> {
    let tmp_path = mktemp();
    {
        let mut contents = String::new();
        write_fmt(&mut contents);
        let mut f = File::create(&tmp_path)?;
        f.write(contents.as_bytes())?;
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
