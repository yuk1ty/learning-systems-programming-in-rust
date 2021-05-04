use std::io;

use signal_hook::{consts::SIGINT, iterator::Signals};

fn main() -> io::Result<()> {
    let mut signals = Signals::new(&[SIGINT])?;

    println!("Waiting SIGINT (CTRL+C)");
    if let Some(_) = signals.into_iter().next() {
        println!("SIGINT arrived");
    }

    Ok(())
}
