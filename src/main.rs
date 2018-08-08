#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
extern crate rand;

use binary_tree::binary_tree;
use grid::Grid;

mod grid;
mod binary_tree;

fn main() {
    let mut grid = Grid::new(5, 5);
    let links = binary_tree(&mut grid);
    grid.set_links(links);
    println!("{}", grid);
}
