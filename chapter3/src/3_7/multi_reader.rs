use std::{
    collections::VecDeque,
    io::{copy, stdout, Read},
};

use lib::io::MultiReader;

fn main() -> std::io::Result<()> {
    let header = "---- HEADER ----\n".as_bytes();
    let content = "Example of MultiReader\n".as_bytes();
    let footer = "---- FOOTER ----\n".as_bytes();
    let mut multi_reader = MultiReader::new(VecDeque::from(vec![
        Box::new(header) as Box<dyn Read>,
        Box::new(content) as Box<dyn Read>,
        Box::new(footer) as Box<dyn Read>,
    ]));
    copy(&mut multi_reader, &mut stdout())?;

    Ok(())
}
