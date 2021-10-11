/// Runs `n` rounds of Blake2b hashing.
fn hash(iter: u128) -> u128 {
    use blake2::{Blake2b, Digest};

    let mut data = String::from("Hello world!");

    let start = std::time::SystemTime::now();
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


fn main() {
    println!("{}", hash(1000));
}
