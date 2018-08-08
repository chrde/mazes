#[macro_use]
extern crate custom_derive;
#[macro_use]
extern crate enum_derive;
extern crate rand;

use binary_tree::binary_tree;
use grid::Grid;
use sidewinder::sidewinder;

mod grid;
mod binary_tree;
mod sidewinder;

fn main() {
    let mut grid = Grid::new(4, 4);
    let _links = binary_tree(&mut grid);
    let links = sidewinder(&mut grid);
    grid.set_links(links);
    println!("{}", grid);
}
