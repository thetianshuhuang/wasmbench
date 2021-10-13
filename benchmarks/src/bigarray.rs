use ndarray::Array2;
use std::time::SystemTime;
use rand::Rng;


fn bigarray(iter: i64) -> i64 {

    let d = 10000;
    let mut arr: Array2<f64> = Array2::zeros((d, d));
    let mut rng = rand::thread_rng();

    let start = SystemTime::now();

    for _ in 1..iter {
        let i = (rng.gen::<u64>() as usize) % d;
        let j = (rng.gen::<u64>() as usize) % d;
        arr[[i, j]] += 1.;
    }

    match start.elapsed() {
        Ok(elapsed) => { elapsed.as_nanos() as i64 }
        Err(_e) => { 0 }
    }
}


#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 { bigarray(i) }
fn main() {}
