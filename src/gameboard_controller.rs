//! Gameboard controller.

use piston::input::GenericEvent;

use Gameboard;

/// Handles events for Sudoku game.
pub struct GameboardController {
    /// Stores the gameboard state.
    pub gameboard: Gameboard,
}

impl GameboardController {
    /// Creates a new gameboard controller.
    pub fn new(gameboard: Gameboard) -> GameboardController {
        let mut gb = GameboardController {
            gameboard: gameboard,
        };
        gb.gameboard.generate_tile();
        gb
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key};
        use gameboard::Direction;

        if let Some(Button::Keyboard(key)) = e.press_args() {
            let was_valid_move = match key {
                Key::Left  => self.gameboard.collapse(Direction::LEFT),
                Key::Right => self.gameboard.collapse(Direction::RIGHT),
                Key::Up    => self.gameboard.collapse(Direction::UP),
                Key::Down  => self.gameboard.collapse(Direction::DOWN),
                _ => false,
            };
            if was_valid_move {
                self.gameboard.generate_tile();
            }
        }
    }
}
