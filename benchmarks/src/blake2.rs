use std::time::SystemTime;

/// Runs `n` rounds of Blake2b hashing.
fn hash(iter: i64) -> u128 {
    use blake2::{Blake2b, Digest};

    let mut data = String::from("Hello world!");

    let start = SystemTime::now();
    for _ in 0..iter {
        let mut hasher = Blake2b::new();
        hasher.update(data);
        data = base64::encode(hasher.finalize().as_slice());
    }
    match start.elapsed() {
        Ok(elapsed) => { elapsed.as_nanos() }
        Err(_e) => { 0 }
    }
}


fn main() {}

#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 {
    hash(i) as i64
}
