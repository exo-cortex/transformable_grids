/// a grid creator with functions to transform grid lines
/// tool to visualize a neural networks transformation of
/// a grid and/or points positioned on it
mod grid;
mod point;
mod straight_line;

use crate::{grid::GridBuilder, point::Point, straight_line::StraightLine};

fn main() {
    let grid = GridBuilder::new()
        .centered_square(Point::default(), 1.0)
        .grid(3);

    grid.print_grid();
}
