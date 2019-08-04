use rand::Rng;
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

/// A position on the rustsweeper cell.
pub type Pos = (u8, u8);

/// The visible state of a rustsweeper cell.
#[derive(PartialEq, Debug)]
pub enum CellView {
    /// The cell is revealed and includes a mine.
    Mine,
    /// The cell is revealed and the number of neighboring mines is visible.
    Empty(u8),
    /// The cell is not revealed.
    Unknown,
}

#[derive(Clone)]
pub struct Cell {
    mine: bool,
    neighbors: u8,
    revealed: bool,
}

impl Cell {
    fn to_view(&self) -> CellView {
        if !self.revealed {
            CellView::Unknown
        } else if self.mine {
            CellView::Mine
        } else {
            CellView::Empty(self.neighbors)
        }
    }
}

/// The current game state of Rustsweeper.
pub struct Model {
    pub width: u8,
    pub height: u8,
    /// If `message` is present, it indicates that the game is over (either won or lost).
    pub message: Option<String>,
    cells: Vec<Cell>,
}

impl Index<Pos> for Model {
    type Output = Cell;

    fn index<'a>(&'a self, (x, y): Pos) -> &'a Self::Output {
        &self.cells[x as usize + y as usize * self.width as usize]
    }
}

impl IndexMut<Pos> for Model {
    fn index_mut<'a>(&'a mut self, (x, y): Pos) -> &'a mut Self::Output {
        &mut self.cells[x as usize + y as usize * self.width as usize]
    }
}

impl Model {
    /// Creates a new instance of the Rustsweeper game with a given width and height.
    pub fn new(width: u8, height: u8) -> Model {
        Model {
            width: width,
            height: height,
            message: None,
            cells: vec![
                Cell {
                    mine: false,
                    neighbors: 0,
                    revealed: false
                };
                usize::from(width) * usize::from(height)
            ],
        }
    }

    /// Resets the game state while preserving the dimensions.
    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.mine = false;
            cell.neighbors = 0;
            cell.revealed = false
        }
        self.message = None
    }

    fn place_mine<R: Rng>(&mut self, rng: &mut R) {
        loop {
            let x: u8 = rng.gen_range(0, self.width);
            let y: u8 = rng.gen_range(0, self.height);
            println!("{} {}", x, y);
            if !self[(x, y)].mine {
                self[(x, y)].mine = true;
                return;
            }
        }
    }

    /// Places the given number of mines randomly on the field.
    pub fn place_mines<R: Rng>(&mut self, number: u8, rng: &mut R) {
        for _ in 0..number {
            self.place_mine(rng)
        }
    }

    /// Calculates the number of neighboring mines of all cells.
    pub fn calc_neighbors(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                for rx in -1..2 {
                    for ry in -1..2 {
                        if let Ok(nx) = u8::try_from(x as i32 + rx) {
                            if let Ok(ny) = u8::try_from(y as i32 + ry) {
                                if nx < self.width && ny < self.height && self[(nx, ny)].mine {
                                    self[(x, y)].neighbors += 1
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn won(&self) -> bool {
        self.cells.iter().all(|cell| cell.revealed || cell.mine)
    }

    /// Reveals the cell at the given position.
    ///
    /// # Return value
    ///
    /// Returns false if the cell was already revealed.
    pub fn reveal(&mut self, pos: Pos) -> bool {
        // let mut cell = &;
        if self[pos].revealed {
            return false;
        };
        self[pos].revealed = true;
        if self[pos].mine {
            self.message = Some(String::from("You lost!"))
        } else if self.won() {
            self.message = Some(String::from("You won!"))
        }
        true
    }

    pub fn to_grid(&self) -> Vec<Vec<CellView>> {
        (0..self.height)
            .map(|y| (0..self.width).map(|x| self[(x, y)].to_view()).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn cell_view() {
        assert_eq!(
            CellView::Mine,
            Cell {
                mine: true,
                neighbors: 4,
                revealed: true
            }
            .to_view()
        );
        assert_eq!(
            CellView::Unknown,
            Cell {
                mine: true,
                neighbors: 4,
                revealed: false
            }
            .to_view()
        );
        assert_eq!(
            CellView::Empty(4),
            Cell {
                mine: false,
                neighbors: 4,
                revealed: true
            }
            .to_view()
        );
        assert_eq!(
            CellView::Unknown,
            Cell {
                mine: false,
                neighbors: 4,
                revealed: false
            }
            .to_view()
        );
    }

    #[test]
    fn new() {
        let model = Model::new(3, 5);
        assert_eq!(model.width, 3);
        assert_eq!(model.height, 5);
        assert_eq!(model.cells.len(), 3 * 5);
        assert!(model.message.is_none())
    }

    #[test]
    fn index() {
        let model = Model::new(3, 5);
        let cell = &model[(0, 1)];
        assert!(!cell.mine);
        assert!(!cell.revealed);
        assert_eq!(cell.neighbors, 0);
    }

    #[test]
    fn reset() {
        let mut model = Model::new(3, 5);
        model[(1, 1)].mine = true;
        model[(1, 1)].revealed = true;
        model[(1, 1)].neighbors = 4;
        model.reset();
        let cell = &model[(1, 1)];
        assert!(!cell.mine);
        assert!(!cell.revealed);
        assert_eq!(cell.neighbors, 0)
    }

    #[test]
    fn place_mine() {
        let mut rng = StdRng::seed_from_u64(23);
        let mut model = Model::new(3, 5);
        model.place_mine(&mut rng);
        assert!(model[(0, 4)].mine);
    }

    #[test]
    fn place_mines() {
        let mut rng = StdRng::seed_from_u64(23);
        let mut model = Model::new(3, 5);
        model.place_mines(4, &mut rng);
        let mines = model
            .cells
            .iter()
            .fold(0, |acc, cell| acc + if cell.mine { 1 } else { 0 });
        assert_eq!(mines, 4)
    }

    macro_rules! assert_neighbors {
        ( $model:ident | $y:ident | ( $( $n:literal ),* ) ) => {{
            let mut x = 0;
            $(
                assert_eq!($model[(x, $y)].neighbors, $n);
                x += 1;
            )*
        }};

        ( $model:ident, $( $x:tt ),* ) => {{
            let mut y = 0;
            $(
                assert_neighbors!($model | y | $x);
                y += 1;
            )*
        }};
    }

    #[test]
    fn calc_neighbors() {
        // 2 2 1
        // X X 2
        // 3 X 2
        // 1 1 1
        // 0 0 0
        let mut model = Model::new(3, 5);
        model[(0, 1)].mine = true;
        model[(1, 1)].mine = true;
        model[(1, 2)].mine = true;
        model.calc_neighbors();
        assert_neighbors!(model, (2, 2, 1), (3, 3, 2), (3, 3, 2), (1, 1, 1), (0, 0, 0))
    }

    #[test]
    fn reveal() {
        let mut model = Model::new(3, 5);
        model[(0, 0)].mine = true;
        assert!(model.reveal((1, 0)));
        assert!(model[(1, 0)].revealed);
        assert!(!model.reveal((1, 0)))
    }

    #[test]
    fn to_grid() {
        let mut model = Model::new(3, 5);
        model[(0, 0)].mine = true;
        model[(1, 0)].mine = true;
        model[(1, 0)].revealed = true;
        model[(2, 0)].revealed = true;
        model[(2, 0)].neighbors = 1;
        let vec = model.to_grid();
        assert_eq!(vec.len(), 5);
        let first_row = vec.first();
        assert!(first_row.is_some());
        assert_eq!(first_row.unwrap().len(), 3);
        assert_eq!(first_row.unwrap()[0], CellView::Unknown);
        assert_eq!(first_row.unwrap()[1], CellView::Mine);
        assert_eq!(first_row.unwrap()[2], CellView::Empty(1));
    }
}
