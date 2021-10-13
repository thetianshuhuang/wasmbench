use std::time::SystemTime;
use nabo::dummy_point::*;
use nabo::KDTree;


fn knn(iter: i64) -> i64 {
    let start = SystemTime::now();

    let cloud = random_point_cloud(1000000);
    let tree = KDTree::new(&cloud);
    for _ in 0..iter {
        let query = random_point();
        let _ = tree.knn(3, &query);        
    }

    match start.elapsed() {
        Ok(elapsed) => { elapsed.as_nanos() as i64 }
        Err(_e) => { 0 }
    }
}


#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 { knn(i) }
fn main() {}
