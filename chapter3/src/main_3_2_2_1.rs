/**
 * 
 * chapter 3_2_2_1
 * io.copy
 * 
 */
use std::io::copy;

pub fn main() -> std::io::Result<()>{
    let mut reader : &[u8] = b"hello!";
    let mut buffer : Vec<u8> = vec![];
  match copy(&mut reader,&mut buffer){
        Ok(result) => result,
        Err(err) => {panic!("ERROR => {}",err)}
    };
    println!("{:?}",&buffer[..]); 
    Ok(())
}