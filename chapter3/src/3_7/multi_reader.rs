use std::{
    collections::VecDeque,
    io::{copy, stdout},
};

use lib::io::MultiReader;

fn main() -> std::io::Result<()> {
    let header = "---- HEADER ----\n".as_bytes();
    let content = "Example of MultiReader\n".as_bytes();
    let footer = "---- FOOTER ----\n".as_bytes();
    let mut multi_reader = MultiReader::new(VecDeque::from(vec![header, content, footer]));
    copy(&mut multi_reader, &mut stdout())?;

    Ok(())
}
