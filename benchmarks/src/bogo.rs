#![feature(asm)]

use std::time::SystemTime;

fn bogo(iter: i64) -> u128 {
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


fn main() {}


#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 {
    bogo(i) as i64
}
