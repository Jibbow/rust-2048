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
    fn iterate(dir: &Direction) -> Box<dyn Iterator<Item=(usize,usize)>> {
        match *dir {
            Direction::UP    => Box::new((0..SIZE*SIZE).map(|z| (z/SIZE, z%SIZE) )),
            Direction::DOWN  => Box::new((0..SIZE*SIZE).map(|z| (z/SIZE, SIZE - z%SIZE - 1) )),
            Direction::LEFT  => Box::new((0..SIZE*SIZE).map(|z| (z%SIZE, z/SIZE) )),
            Direction::RIGHT => Box::new((0..SIZE*SIZE).map(|z| (SIZE - z%SIZE - 1, z/SIZE) )),
        }
    }

    fn is_valid_pos(pos: (usize,usize)) -> bool {
        pos.0 >= 0 && (pos.0 as usize) < SIZE
            && pos.1 >= 0 && (pos.1 as usize) < SIZE
    }

    fn add(first: (usize, usize), second: (isize, isize)) -> (usize,usize) {
        (((first.0 as isize) + second.0) as usize, ((first.1 as isize) + second.1) as usize)
    }

    /// Processes all tiles on the game board and moves them around or collapses them.
    pub fn collapse(&mut self, dir: Direction) -> bool {
        // will be set, if a valid move has been made in this turn
        let mut was_valid_move = false;

        // holds a blocking position which can not be passed by other tiles.
        // Reason: when two tiles have been collapsed no subsequent collapsed
        //         should be done with this tile.
        //         Because all tiles are processed in a special order, it is 
        //         enough to just have one blocking field that gets updated
        //         when two tiles are collapsed.
        let mut blocker = (<usize>::max_value(),usize::max_value());

        // processes all tiles: moves them and collapses them
        for pos in Gameboard::iterate(&dir) {
            // skip if cell is empty
            if self.get(&pos) == 0 {
                continue;
            }

            // current position of the processed tile
            let mut source_pos = pos.clone();
            // cell where the current tile should move to
            let mut target_pos = Gameboard::add(source_pos, dir.value());

            // move the current tile
            while Gameboard::is_valid_pos(target_pos) && target_pos != blocker {
                // if there is no other tile -> move
                if self.get(&target_pos) == 0 {
                    let target_value = self.get(&source_pos);
                    self.set(&target_pos, target_value);
                    self.set(&source_pos, 0);

                    was_valid_move = true;
                } else { // there is another tile in the way!
                    // if the other tile has the same value -> collapse
                    if self.get(&target_pos) == self.get(&source_pos) {
                        // double the value of the collapsed cell
                        let target_value = self.get(&target_pos) * 2;
                        self.set(&target_pos, target_value);
                        self.set(&source_pos, 0);
                        // update the blocker position -> no collapses with this cell are allowed
                        blocker = target_pos.clone();

                        was_valid_move = true;
                    }
                    break;
                }
                // update positions after move
                source_pos = target_pos;
                target_pos = Gameboard::add(source_pos, dir.value())
            }
        }
        was_valid_move
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

