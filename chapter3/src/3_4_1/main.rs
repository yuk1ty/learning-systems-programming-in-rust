use std::io::{stdin, Error, ErrorKind, Read};

fn main() {
    loop {
        let mut buffer = vec![0; 5];

        match stdin().read(&mut buffer) {
            Ok(size) if size == 0 => {
                println!("EOF");
                break;
            }
            Ok(size) => {
                println!(
                    "size={} input'{}'",
                    size,
                    std::str::from_utf8(&buffer).unwrap()
                ); // rustはStringがUTF-8だから若干違うのはしょうがない？
            }
            Err(e) => {
                if e.kind() == ErrorKind::UnexpectedEof {
                    println!("EOF");
                    break;
                } else {
                    unreachable!();
                }
            }
        }
    }
}
