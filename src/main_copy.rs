use ndarray::prelude::*;
fn main() {
    let grid = Array::from_iter(0..100).into_shape((10, 10)).unwrap();

    println!("{:?}", grid.slice(s![7..10, 4..7]));

    for (i, j) in grid.indexed_iter().filter_map(|(index, &value)| (value == 85).then(|| index)){
        println!("{:?}", i);
        println!("{:?}", j);
    }
}
