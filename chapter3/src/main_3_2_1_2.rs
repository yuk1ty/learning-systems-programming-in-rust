/**
 * 
 * chapter 3_2_1_2
 * io.read
 * 
 */

use std::io::Read;

pub fn main() -> std::io::Result<()>{
    let mut r : &[u8] = b"abcdefg";
    let mut buffer = [0;4];
    match r.read_exact(&mut buffer){
        Ok(result) => result,
        Err(err) => {panic!("ERROR => {}",err)}
    };//buffer.len() < r => ERROR
    println!("{:?}",&buffer[..4]); 
    Ok(())
 }