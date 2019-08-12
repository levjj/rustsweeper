use rand::{Rng, thread_rng};
use std::convert::TryFrom;
use std::ops::{Index, IndexMut};

/// A position on the rustsweeper cell.
pub type Pos = (u8, u8);

#[derive(Clone, Debug, PartialEq)]
pub enum CellState {
    Marked,
    Unmarked,
    Revealed,
}

impl Default for CellState {
    fn default() -> Self {
        CellState::Unmarked
    }
}

#[derive(Clone)]
pub struct Cell {
    pub mine: bool,
    pub neighbors: u8,
    pub state: CellState,
}

impl Cell {
    fn reset(&mut self) {
        self.mine = false;
        self.neighbors = 0;
        self.state = CellState::Unmarked;
    }
}

/// The current game state of Rustsweeper.
pub struct Model {
    pub width: u8,
    pub height: u8,
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

const NEIGHBOR_POS: &'static [(i32, i32); 8] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Model {
    /// Creates a new instance of the Rustsweeper game with a given width and height.
    pub fn new(width: u8, height: u8) -> Model {
        Model {
            width: width,
            height: height,
            cells: vec![
                Cell {
                    state: CellState::Unmarked,
                    mine: false,
                    neighbors: 0,
                };
                usize::from(width) * usize::from(height)
            ],
        }
    }

    /// Resets the game state while preserving the dimensions.
    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut() {
            cell.reset();
        }
    }

    fn place_mine<R: Rng>(&mut self, rng: &mut R) {
        loop {
            let x: u8 = rng.gen_range(0, self.width);
            let y: u8 = rng.gen_range(0, self.height);
            if !self[(x, y)].mine {
                self[(x, y)].mine = true;
                break;
            }
        }
    }

    /// Places the given number of mines randomly on the field.
    pub fn place_mines<R: Rng>(&mut self, number: u8, rng: &mut R) {
        for _ in 0..number {
            self.place_mine(rng)
        }
    }

    fn iter_neighbors(&self, (x, y): Pos) -> impl Iterator<Item = (u8, u8)> {
        let width = self.width;
        let height = self.height;
        NEIGHBOR_POS.iter().filter_map(move |(rx, ry)| {
            match (u8::try_from(x as i32 + rx), u8::try_from(y as i32 + ry)) {
                (Ok(unx), Ok(uny)) if unx < width && uny < height => Some((unx, uny)),
                _ => None
            }
        })
    }

    /// Calculates the number of neighboring mines of all cells.
    pub fn calc_neighbors(&mut self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self[(x, y)].neighbors = self
                    .iter_neighbors((x, y))
                    .filter(|&n| self[n].mine)
                    .count() as u8
            }
        }
    }

    fn lost(&self) -> bool {
        self.cells
            .iter()
            .any(|cell| cell.state == CellState::Revealed && cell.mine)
    }

    fn won(&self) -> bool {
        self.cells
            .iter()
            .all(|cell| cell.state == CellState::Revealed || cell.mine)
    }

    /// Whether the game is won or lost.
    pub fn game_over(&self) -> bool {
        self.lost() || self.won()
    }

    /// Returns a message that summarizes the game state.
    pub fn message(&self) -> String {
        if self.lost() {
            String::from("Game lost!")
        } else if self.won() {
            String::from("Game won!")
        } else {
            let marked = self
                .cells
                .iter()
                .filter(|cell| cell.state == CellState::Marked)
                .count();
            let mines = self.cells.iter().filter(|cell| cell.mine).count();
            format!("Found {} of {} mines.", marked, mines)
        }
    }

    fn reveal_transitive(&mut self, pos: Pos, todo: &mut Vec<Pos>) {
        for neighbor in self.iter_neighbors(pos) {
            if self[neighbor].state == CellState::Unmarked {
                self[neighbor].state = CellState::Revealed;
                if self[neighbor].neighbors == 0 {
                    todo.push(neighbor)
                }
            }
        }
    }

    /// Reveals the cell at the given position and transitively reveals all other connected cells
    /// with 0 neighboring mines.
    pub fn reveal(&mut self, pos: Pos) {
        self[pos].state = CellState::Revealed;
        if self[pos].neighbors == 0 && !self[pos].mine {
            let mut todo = vec![pos];
            while let Some(next) = todo.pop() {
                self.reveal_transitive(next, &mut todo);
            }
        }
    }

    pub fn toggle_marked(&mut self, pos: Pos) {
        self[pos].state = match self[pos].state {
            CellState::Marked => CellState::Unmarked,
            CellState::Unmarked => CellState::Marked,
            CellState::Revealed => CellState::Revealed,
        }
    }

    pub fn to_grid(&self) -> Vec<Vec<Cell>> {
        (0..self.height)
            .map(|y| (0..self.width).map(|x| self[(x, y)].clone()).collect())
            .collect()
    }

    pub fn prepare_mines(&mut self, number_of_mines: u8) {
        self.place_mines(number_of_mines, &mut thread_rng());
        self.calc_neighbors();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn new() {
        let model = Model::new(3, 5);
        assert_eq!(model.width, 3);
        assert_eq!(model.height, 5);
        assert_eq!(model.cells.len(), 3 * 5);
    }

    #[test]
    fn index() {
        let model = Model::new(3, 5);
        let cell = &model[(0, 1)];
        assert!(!cell.mine);
        assert_eq!(cell.state, CellState::Unmarked);
        assert_eq!(cell.neighbors, 0);
    }

    #[test]
    fn reset() {
        let mut model = Model::new(3, 5);
        model[(1, 1)].mine = true;
        model[(1, 1)].state = CellState::Revealed;
        model[(1, 1)].neighbors = 4;
        model.reset();
        let cell = &model[(1, 1)];
        assert!(!cell.mine);
        assert_eq!(cell.state, CellState::Unmarked);
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
        let mines = model.cells.iter().filter(|cell| cell.mine).count();
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
        assert_neighbors!(model, (2, 2, 1), (2, 2, 2), (3, 2, 2), (1, 1, 1), (0, 0, 0))
    }

    #[test]
    fn reveal() {
        let mut model = Model::new(3, 5);
        model[(0, 0)].mine = true;
        model[(1, 0)].neighbors = 1;
        model.reveal((1, 0));
        assert_eq!(model[(1, 0)].state, CellState::Revealed);
        assert_eq!(model[(2, 0)].state, CellState::Unmarked);
        model.reveal((2, 0));
        assert_eq!(model[(2, 0)].state, CellState::Revealed);
        assert_eq!(model[(2, 1)].state, CellState::Revealed);
    }

    #[test]
    fn toggle_marked() {
        let mut model = Model::new(3, 5);
        model[(0, 0)].mine = true;
        model.toggle_marked((1, 0));
        assert_eq!(model[(1, 0)].state, CellState::Marked);
        assert_eq!(model[(2, 0)].state, CellState::Unmarked);
        model.toggle_marked((2, 0));
        assert_eq!(model[(2, 0)].state, CellState::Marked);
        assert_eq!(model[(2, 1)].state, CellState::Unmarked);
    }

    #[test]
    fn to_grid() {
        let model = Model::new(3, 5);
        let vec = model.to_grid();
        assert_eq!(vec.len(), 5);
        let first_row = vec.first();
        assert!(first_row.is_some());
        assert_eq!(first_row.unwrap().len(), 3);
    }
}
