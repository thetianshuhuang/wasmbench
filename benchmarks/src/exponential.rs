use ndarray::Array;
use std::time::SystemTime;
use rand::Rng;


fn exponential(iter: i64) -> i64 {

    let d = 100;

    let mut x = Array::zeros((d, d));
    let mut rng = rand::thread_rng();
    for i in 0..d {
        for j in 0..d {
            x[[i, j]] = rng.gen::<f64>();
        }
    }

    let start = SystemTime::now();

    let mut acc = Array::zeros((d, d));
    for i in 0..d { acc[[i, i]] = 1.; }
    let mut x_i = x.clone();

    for k in 1..iter {
        acc = &acc + &x_i;
        x_i = &x_i.dot(&x) / (k as f64);
    }

    match start.elapsed() {
        Ok(elapsed) => { elapsed.as_nanos() as i64 }
        Err(_e) => { 0 }
    }
}


#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 { exponential(i) }
fn main() {}
