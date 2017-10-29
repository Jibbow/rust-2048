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
        GameboardController {
            gameboard: gameboard,
        }
    }

    /// Handles events.
    pub fn event<E: GenericEvent>(&mut self, e: &E) {
        use piston::input::{Button, Key};

        if let Some(Button::Keyboard(key)) = e.press_args() {
            match key {
                Key::Left  => self.gameboard.move_left(),
                Key::Right => self.gameboard.move_right(),
                Key::Up    => self.gameboard.move_up(),
                Key::Down  => self.gameboard.move_down(),
                _ => {}
            }
        }
    }
}
