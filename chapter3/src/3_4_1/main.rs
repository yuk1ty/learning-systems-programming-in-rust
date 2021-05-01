use std::io::{stdin, Read};

fn main() {
    loop {
        let mut buffer = vec![0; 5];

        let size = stdin().read(&mut buffer).unwrap();
        if size == 0 {
            println!("EOF");
            break;
        }

        println!(
            "size={} input'{}'",
            size,
            std::str::from_utf8(&buffer).unwrap()
        ); // rustはStringがUTF-8だから若干違うのはしょうがない？
    }
}
