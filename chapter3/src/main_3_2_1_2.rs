/**
 * 
 * chapter 3_2_1_2
 * io.read
 * 
 */

use std::io::Read;
use std::fs::File;

pub fn main() -> std::io::Result<()>{
    let mut r = File::open("./chapter3/content/bar")?;
    let mut buffer = [0;4];
    match r.read_exact(&mut buffer){
        Ok(result) => result,
        Err(err) => {panic!("ERROR => {}",err)}
    };//Error!
    println!("{:?}",&buffer[..4]); 
    Ok(())
 }