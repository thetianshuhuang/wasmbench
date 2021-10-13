use std::time::SystemTime;


fn bogo(iter: i64) -> i64 {
    let start = SystemTime::now();
    // We do this instead of just spinning in order to trick the higher
    // levels of compiler optimization into not collapsing this loop
    let mut i: i64 = 0;
    for j in 0..iter { i ^= j; }
    for j in 0..iter { i ^= j; }
    match start.elapsed() {
        Ok(elapsed) => { (elapsed.as_nanos() as i64) + i }
        Err(_e) => { i }
    }
}


fn main() {}


#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 { bogo(i) }
