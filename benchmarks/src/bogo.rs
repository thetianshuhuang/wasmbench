#![feature(asm)]

use std::time::SystemTime;

fn bogo(iter: u128) -> u128 {
    let start = SystemTime::now();
    for _ in 0..iter {
        unsafe {
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
            asm!("nop");
        }
    }
    match start.elapsed() {
        Ok(elapsed) => { elapsed.as_nanos() }
        Err(_e) => { 0 }
    }
}


fn main() {
    println!("{}", bogo(1000));
}
