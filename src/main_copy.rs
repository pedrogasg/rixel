use ndarray::prelude::*;
pub mod movement;

fn main() {
    let grid = Array::from_iter(0..100).into_shape((10, 10)).unwrap();

    println!("{:?}", grid.slice(s![7..10, 4..7]));

    let ax = movement::Actions::new(grid);

    println!("{:?}", ax.action_grid);
    println!("{:?}", ax.action_grid.slice(s![8..11, 5..8]));
    let test = ax.get_shifts(8, 5);

    println!("{:?}", test.top);
    println!("{:?}", test.left);
    println!("{:?}", test.bottom);
    println!("{:?}", test.right);
}
