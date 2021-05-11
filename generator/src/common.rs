pub struct Maze {
    cells: Vec<Cell>,
    width: usize,
}

impl Maze {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = (0..(width * height))
            .into_iter()
            .map(|_| Cell::default())
            .collect();
        Self { cells, width }
    }

    pub fn cell(&self, pos: usize) -> &Cell {
        &self.cells[pos]
    }

    pub fn middle_cell(&self) -> usize {
        self.len() / 2 + self.width / 2
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn len(&self) -> usize {
        self.cells.len()
    }

    pub fn neighbor_at(&self, cell: usize, n: Neighbor) -> Option<Neighbor1> {
        match n {
            Neighbor::North => {
                // reject cells on the first row
                if cell < self.width {
                    None
                } else {
                    Some(Neighbor1 {
                        dir: n,
                        idx: cell - self.width,
                    })
                }
            }
            Neighbor::South => {
                // reject cells on the last row
                let c = cell + self.width;
                if c >= self.cells.len() {
                    None
                } else {
                    Some(Neighbor1 { dir: n, idx: c })
                }
            }
            Neighbor::East => {
                // reject cells on the last column
                if (cell + 1) % self.width == 0 {
                    None
                } else {
                    Some(Neighbor1 {
                        dir: n,
                        idx: cell + 1,
                    })
                }
            }
            Neighbor::West => {
                // reject cells on the first column
                if cell % self.width == 0 {
                    None
                } else {
                    Some(Neighbor1 {
                        dir: n,
                        idx: cell - 1,
                    })
                }
            }
        }
    }
    pub fn unlink(&mut self, cell: usize, n: Neighbor) {
        let neighbor = self.neighbor_at(cell, n).unwrap();
        self.cells[cell].unlink(n);
        self.cells[neighbor.idx].unlink(n.opposite());
    }


    pub fn link(&mut self, cell: usize, n: Neighbor) {
        let neighbor = self.neighbor_at(cell, n).unwrap();
        self.cells[cell].link(n);
        self.cells[neighbor.idx].link(n.opposite());
    }

    pub fn linked_neighbors<'a>(&'a self, cell: usize) -> Vec<Neighbor1> {
        Neighbor::iter()
            .iter()
            .cloned()
            .filter_map(|dir| self.neighbor_at(cell, dir))
            .filter(|n| {
                self.cell(cell).has_link(n.dir) && self.cell(n.idx).has_link(n.dir.opposite())
            })
            .collect()
    }

    pub fn neighbors(&self, cell: usize) -> Neighbors {
        Neighbors {
            inner: [
                self.neighbor_at(cell, Neighbor::North),
                self.neighbor_at(cell, Neighbor::South),
                self.neighbor_at(cell, Neighbor::East),
                self.neighbor_at(cell, Neighbor::West),
            ],
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Neighbor1 {
    pub dir: Neighbor,
    pub idx: usize,
}

#[derive(Clone, Copy, Debug)]
pub enum Neighbor {
    North,
    South,
    East,
    West,
}

impl Neighbor {
    pub fn iter() -> &'static [Self] {
        &[
            Neighbor::North,
            Neighbor::South,
            Neighbor::East,
            Neighbor::West,
        ]
    }

    pub fn opposite(self) -> Self {
        match self {
            Neighbor::North => Neighbor::South,
            Neighbor::South => Neighbor::North,
            Neighbor::East => Neighbor::West,
            Neighbor::West => Neighbor::East,
        }
    }
}

#[derive(Default, Debug)]
pub struct Cell {
    links: Links,
}

impl Cell {
    pub fn links(&self) -> Links {
        self.links
    }

    pub fn unlink(&mut self, dir: Neighbor) {
        match dir {
            Neighbor::North => self.links.north = false,
            Neighbor::South => self.links.south = false,
            Neighbor::East => self.links.east = false,
            Neighbor::West => self.links.west = false,
        }
    }

    pub fn link(&mut self, dir: Neighbor) {
        match dir {
            Neighbor::North => self.links.north = true,
            Neighbor::South => self.links.south = true,
            Neighbor::East => self.links.east = true,
            Neighbor::West => self.links.west = true,
        }
    }

    pub fn has_link(&self, dir: Neighbor) -> bool {
        match dir {
            Neighbor::North => self.links.north,
            Neighbor::South => self.links.south,
            Neighbor::East => self.links.east,
            Neighbor::West => self.links.west,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Neighbors {
    inner: [Option<Neighbor1>; 4],
}

impl Neighbors {
    // pub fn iter(&self) -> impl Iterator<Item = usize> + '_ {
    //     self.inner.iter().filter_map(|x| *x)
    // }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Links {
    pub north: bool,
    pub south: bool,
    pub east: bool,
    pub west: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn neighbor_at() {
        let t = BinaryTree::new(7, 3);
        for c in 0..t.cells.len() {
            println!("{}: {:?}", c, t.neighbors(c));
        }
        panic!()
    }
}
