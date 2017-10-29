//! Game board logic

extern crate rand;

/// Size of game board
const SIZE: usize = 4;

/// Stores game board information
pub struct Gameboard {
    /// Stores the content of the cells
    /// '0' is an empty cell
    pub cells: [[u16; SIZE]; SIZE],
}

pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    fn value(&self) -> (isize,isize) {
        match *self {
            Direction::UP => (0,-1),
            Direction::DOWN => (0,1),
            Direction::LEFT => (-1,0),
            Direction::RIGHT => (1,0),
        }
    }
}




impl Gameboard {

    /// Creates a new game board
    pub fn new() -> Gameboard {
        Gameboard {
            cells: [[0; SIZE]; SIZE],
        }
    }

    /// Gets a cell value
    fn get(&self, pos: &(usize,usize)) -> u16 {
        self.cells[pos.0][pos.1]
    }

    /// Sets a cell value
    fn set(&mut self, pos: &(usize,usize), value: u16) {
            self.cells[pos.0][pos.1] = value;
    }

    /// Returns an iterator which iterates through every tile in the correct order
    /// depending on the moving direction of the tiles.
    fn iterate(dir: &Direction) -> Box<Iterator<Item=(usize,usize)>> {
        match *dir {
            Direction::UP    => Box::new((0..SIZE*SIZE).map(|z| (z/SIZE, z%SIZE) )),
            Direction::DOWN  => Box::new((0..SIZE*SIZE).map(|z| (z/SIZE, SIZE - z%SIZE - 1) )),
            Direction::LEFT  => Box::new((0..SIZE*SIZE).map(|z| (z%SIZE, z/SIZE) )),
            Direction::RIGHT => Box::new((0..SIZE*SIZE).map(|z| (z%SIZE, SIZE - z/SIZE - 1) )),
        }
    }

    fn is_valid_pos(pos: (usize,usize)) -> bool {
        pos.0 >= 0 && (pos.0 as usize) < SIZE
            && pos.1 >= 0 && (pos.1 as usize) < SIZE
    }

    fn add(first: (usize, usize), second: (isize, isize)) -> (usize,usize) {
        (((first.0 as isize) + second.0) as usize, ((first.1 as isize) + second.1) as usize)
    }

    pub fn collapse(&mut self, dir: Direction) {
        println!("");
        for pos in Gameboard::iterate(&dir) {
            println!("{:?}", pos);
            let mut tmp_pos = pos.clone();
            while Gameboard::is_valid_pos(Gameboard::add(tmp_pos, dir.value())) {
                // move cell
                if self.get(&Gameboard::add(tmp_pos, dir.value())) == 0 {
                    let val = self.get(&tmp_pos);
                    self.set(&Gameboard::add(tmp_pos, dir.value()), val);
                    self.set(&tmp_pos, 0);
                } 
                // collapse two cells
                if self.get(&Gameboard::add(tmp_pos, dir.value())) == self.get(&tmp_pos) {
                    let val = self.get(&tmp_pos) * 2;
                    self.set(&Gameboard::add(tmp_pos, dir.value()), val);
                    self.set(&tmp_pos, 0);
                    break;
                }
                tmp_pos = Gameboard::add(tmp_pos, dir.value())
            }
        }
    }

    /// Randomly generates a new tile on a free field on the game board.
    /// Returns true, if a tile has successfully been created, otherwise false.
    /// A tile can not be created if there is no free space field left.
    pub fn generate_tile(&mut self) -> bool {
        use self::rand::Rng;

        let free: Vec<(usize,usize)> = Gameboard::iterate(&Direction::UP).filter(|x| self.get(x) == 0).collect();
        if free.len() != 0 {
            let i = rand::thread_rng().gen_range(0, free.len());
            
            self.cells[free[i].0][free[i].1] = 2;
            return true
        }
        false
    }
}

