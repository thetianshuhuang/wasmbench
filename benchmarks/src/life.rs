use std::time::SystemTime;

// From ndarray examples
use ndarray::prelude::*;

const INPUT: &[u8] = include_bytes!("life.txt");

const N: usize = 100;

type Board = Array2<u8>;

fn parse(x: &[u8]) -> Board {
    // make a border of 0 cells
    let mut map = Board::from_elem(((N + 2), (N + 2)), 0);
    let a = Array::from_iter(x.iter().filter_map(|&b| match b {
        b'#' => Some(1),
        b'.' => Some(0),
        _ => None,
    }));

    let a = a.into_shape((N, N)).unwrap();
    map.slice_mut(s![1..-1, 1..-1]).assign(&a);
    map
}

// Rules
//
// 2 or 3 neighbors: stay alive
// 3 neighbors: birth
// otherwise: death

fn iterate(z: &mut Board, scratch: &mut Board) {
    // compute number of neighbors
    let mut neigh = scratch.view_mut();
    neigh.fill(0);
    neigh += &z.slice(s![0..-2, 0..-2]);
    neigh += &z.slice(s![0..-2, 1..-1]);
    neigh += &z.slice(s![0..-2, 2..]);

    neigh += &z.slice(s![1..-1, 0..-2]);
    neigh += &z.slice(s![1..-1, 2..]);

    neigh += &z.slice(s![2.., 0..-2]);
    neigh += &z.slice(s![2.., 1..-1]);
    neigh += &z.slice(s![2.., 2..]);

    // birth where n = 3 and z[i] = 0,
    // survive where n = 2 || n = 3 and z[i] = 1
    let mut zv = z.slice_mut(s![1..-1, 1..-1]);

    // this is autovectorized amazingly well!
    zv.zip_mut_with(&neigh, |y, &n| *y = ((n == 3) || (n == 2 && *y > 0)) as u8);
}

fn turn_on_corners(z: &mut Board) {
    let n = z.nrows();
    let m = z.ncols();
    z[[1, 1]] = 1;
    z[[1, m - 2]] = 1;
    z[[n - 2, 1]] = 1;
    z[[n - 2, m - 2]] = 1;
}


fn life(iter: i64) -> i64 {
    let mut a = parse(INPUT);
    let mut scratch = Board::zeros((N, N));
    turn_on_corners(&mut a);

    let start = SystemTime::now();
    for _ in 0..iter {
        iterate(&mut a, &mut scratch);
        turn_on_corners(&mut a);
    }
    match start.elapsed() {
        Ok(elapsed) => { elapsed.as_nanos() as i64 }
        Err(_e) => { 0 }
    }
}


#[no_mangle]
pub extern fn benchmark(i: i64) -> i64 { life(i) }
fn main() {}
