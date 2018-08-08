use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt;
use std::i16;

#[derive(Debug)]
pub struct Cell {
    id: i16,
    position: Position,
}

impl From<(i8, i8)> for Position {
    fn from(p: (i8, i8)) -> Self {
        Position { row: p.0, column: p.1 }
    }
}

impl Cell {
    pub fn new<T: Into<Position>>(id: i16, position: T) -> Self {
        Cell { id, position: position.into() }
    }

    pub fn id(&self) -> i16 {
        self.id
    }
}

custom_derive! {
    #[derive(Copy, Clone, Debug, PartialEq, Eq, IterVariants(DirectionVariants))]
    pub enum Direction { NORTH, SOUTH, EAST, WEST, }
}

pub struct Grid {
    rows: i8,
    columns: i8,
    cells: Vec<Cell>,
    links: HashSet<(i16, i16)>,
}

impl fmt::Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();
        assert_eq!(self.columns as usize * self.rows as usize, self.cells.len());
        let mut cells = self.cells().iter();
        s.push_str("+");
        for _ in 0..self.columns {
            s.push_str("---+");
        }
        for _ in 0..self.rows {
            let mut top = "\n|".to_string();
            let mut bottom = "\n+".to_string();
            for _ in 0..self.columns {
                top.push_str("   ");
                let cell = cells.next().unwrap();
                if self.is_linked(cell, Direction::EAST) {
                    top.push_str(" ");
                } else {
                    top.push_str("|");
                }
                if self.is_linked(cell, Direction::SOUTH) {
                    bottom.push_str("   ");
                } else {
                    bottom.push_str("---");
                }
                bottom.push_str("+");
            }
            s.push_str(&top);
            s.push_str(&bottom);
        }
        write!(f, "{}", s)
    }
}

impl Grid {
    pub fn new(rows: i8, columns: i8) -> Self {
        let mut ids = 0..i16::MAX;
        let mut cells = Vec::with_capacity((rows * columns) as usize);
        for r in 0..rows {
            for c in 0..columns {
                let cell = Cell::new(ids.next().unwrap(), (r, c));
                cells.push(cell);
            }
        }

        Grid { rows, columns, cells, links: HashSet::new() }
    }

    pub fn cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn is_valid(&self, position: Position) -> bool {
        let row_valid = position.row >= 0 && position.row < self.rows;
        let column_valid = position.column >= 0 && position.column < self.columns;
        row_valid && column_valid
    }

    pub fn neighbor<T: Borrow<Cell>>(&self, cell: T, direction: Direction) -> Option<&Cell> {
        let cell = cell.borrow();
        self.move_pos(cell.position, direction).and_then(|p| {
            let index = p.row * self.columns + p.column;
            self.cells.get(index as usize)
        })
    }

    pub fn set_links(&mut self, links: HashSet<(i16, i16)>) {
        self.links = links;
    }

    pub fn is_linked<T: Borrow<Cell>>(&self, cell: T, direction: Direction) -> bool {
        let cell = cell.borrow();
        if let Some(n) = self.neighbor(cell, direction) {
            self.links.contains(&(cell.id(), n.id()))
        } else {
            false
        }
    }

    fn move_pos(&self, position: Position, direction: Direction) -> Option<Position> {
        let new_pos = match direction {
            Direction::EAST => Position { column: position.column + 1, ..position },
            Direction::WEST => Position { column: position.column - 1, ..position },
            Direction::NORTH => Position { row: position.row - 1, ..position },
            Direction::SOUTH => Position { row: position.row + 1, ..position },
        };
        if self.is_valid(new_pos) {
            Some(new_pos)
        } else {
            None
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Position {
    row: i8,
    column: i8,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn grid(size: i8) -> Grid {
        Grid::new(size, size)
    }

    fn assert_neighbors(grid: &Grid, cell: Cell, directions: &[Direction]) {
        for dir in Direction::iter_variants() {
            if directions.contains(&dir) {
                assert!(grid.neighbor(&cell, dir).is_some(), format!("Neighbor at {:?} of {:?} should exist", &dir, &cell))
            } else {
                assert!(grid.neighbor(&cell, dir).is_none(), format!("Neighbor at {:?} of {:?} should not exist", &dir, &cell))
            }
        }
    }

    #[test]
    fn cells_have_right_size() {
        let grid = grid(4);
        assert_eq!(16, grid.cells().len());
    }

    #[test]
    fn cells_have_right_id() {
        let grid = grid(4);
        for (i, x) in grid.cells().iter().enumerate() {
            assert_eq!(i, x.id as usize, "{:?} is invalid", x)
        }
    }

    #[test]
    fn cells_are_valid() {
        let grid = grid(4);
        for cell in grid.cells() {
            assert!(grid.is_valid(cell.position), format!("{:?} is invalid", cell))
        }
    }

    #[test]
    fn corner_cells_dont_have_some_neighbors() {
        let grid = grid(4);
        assert_neighbors(&grid, Cell::new(0, (0, 0)), &[Direction::EAST, Direction::NORTH]);
        assert_neighbors(&grid, Cell::new(0, (3, 3)), &[Direction::WEST, Direction::SOUTH]);
        assert_neighbors(&grid, Cell::new(0, (0, 3)), &[Direction::WEST, Direction::NORTH]);
        assert_neighbors(&grid, Cell::new(0, (3, 0)), &[Direction::EAST, Direction::SOUTH]);
    }

    #[test]
    fn side_cells_dont_have_some_neighbors() {
        let grid = grid(3);
        assert_neighbors(&grid, Cell::new(0, (0, 1)), &[Direction::NORTH, Direction::WEST, Direction::EAST]);
        assert_neighbors(&grid, Cell::new(0, (1, 0)), &[Direction::NORTH, Direction::SOUTH, Direction::EAST]);
        assert_neighbors(&grid, Cell::new(0, (2, 1)), &[Direction::SOUTH, Direction::WEST, Direction::EAST]);
        assert_neighbors(&grid, Cell::new(0, (1, 2)), &[Direction::NORTH, Direction::SOUTH, Direction::WEST]);
    }
}