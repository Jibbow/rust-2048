//! Game board logic

/// Size of game board
const SIZE: usize = 4;

/// Stores game board information
pub struct Gameboard {
    /// Stores the content of the cells
    /// '0' is an empty cell
    pub cells: [[u16; SIZE]; SIZE],
}

impl Gameboard {
    /// Creates a new game board
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }

    /// Set cell value
    fn set(&mut self, pos: [usize; 2], value: u16) {
        self.cells[pos[0]][pos[1]] = value;
    }

    pub fn move_left(&mut self) { self.set([0,0],64); println!("moving left"); }
    pub fn move_right(&mut self) { println!("moving right"); }
    pub fn move_up(&mut self) { println!("moving up"); }
    pub fn move_down(&mut self) { println!("moving down"); }
}


