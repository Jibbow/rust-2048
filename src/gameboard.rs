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

    fn get(&self, pos: &(usize,usize)) -> u16 {
        self.cells[pos.0][pos.1]
    }

    /// Set cell value
    fn set(&mut self, pos: &(usize,usize), value: u16) {
            self.cells[pos.0][pos.1] = value;
    }

    fn iterate<'a>(dir: &Direction) -> Box<Iterator<Item=(usize,usize)> + 'a> {
        match dir {
            UP    => Box::new((0..SIZE*SIZE).map(|z| (z/SIZE, z%SIZE) )),
            DOWN  => Box::new((0..SIZE*SIZE).map(|z| (z/SIZE, SIZE - z%SIZE) )),
            LEFT  => Box::new((0..SIZE*SIZE).map(|z| (z%SIZE, z/SIZE) )),
            RIGHT => Box::new((0..SIZE*SIZE).map(|z| (z%SIZE, SIZE - z/SIZE) )),
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
        for pos in Gameboard::iterate(&dir) {
            let mut tmp_pos = pos.clone();
            while Gameboard::is_valid_pos(Gameboard::add(tmp_pos, dir.value())) {
                if self.get(&Gameboard::add(tmp_pos, dir.value())) == 0 {
                    // move cell
                    let val = self.get(&tmp_pos);
                    self.set(&Gameboard::add(tmp_pos, dir.value()), val);
                    self.set(&tmp_pos, 0);
                }
                tmp_pos = Gameboard::add(tmp_pos, dir.value())
            }
        }
    }

    pub fn generate_tile(&mut self) {
        use self::rand::Rng;

        let free: Vec<(usize,usize)> = Gameboard::iterate(&Direction::UP).filter(|x| self.get(x) == 0).collect();
        if free.len() != 0 {
            let i = rand::thread_rng().gen_range(0, free.len());
            
            self.cells[free[i].0][free[i].1] = 2;
        }
    }
}

