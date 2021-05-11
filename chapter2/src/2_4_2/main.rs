use std::io::{stdout, Write};

fn main() -> std::io::Result<()> {
    stdout().write_all(b"std::io::stdout example\n")?;
    Ok(())
}
