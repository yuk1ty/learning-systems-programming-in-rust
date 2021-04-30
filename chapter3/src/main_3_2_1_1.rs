/**
 * 
 * chapter 3_2_1_1
 * io.read
 * 
 */

use std::io::Read;
use std::fs::File;

pub fn main() -> std::io::Result<()>{
    let mut r = File::open("./chapter3/content/foo")?;
    let mut buffer : Vec<u8>= vec!();
    let size = match r.read_to_end(&mut buffer){
        Ok(size) => size,
        Err(err) => {return Err(err.into());}
    };
    // println!("{:?}",&buffer[..size]); 
    Ok(())
 }