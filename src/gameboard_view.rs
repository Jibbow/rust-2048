//! Gameboard view.

use graphics::types::Color;
use graphics::{Context, Graphics};

use GameboardController;

/// Stores gameboard view settings.
pub struct GameboardViewSettings {
    /// Position from left-top corner.
    pub position: [f64; 2],
    /// Size of gameboard along horizontal and vertical edge.
    pub size: f64,
    /// Background color.
    pub board_background_color: Color,
    /// Edge color around the whole board.
    pub board_edge_color: Color,
    /// Edge radius around the whole board.
    pub board_edge_radius: f64,
    /// Edge color of single cell
    pub cell_edge_color: Color,
    /// Edge radius between cells.
    pub cell_edge_radius: f64,
}

impl GameboardViewSettings {
    /// Creates new gameboard view settings.
    pub fn new() -> GameboardViewSettings {
        GameboardViewSettings {
            position: [0.0; 2],
            size: 400.0,
            board_background_color: [0.8, 0.8, 1.0, 1.0],
            board_edge_color: [0.0, 0.0, 0.2, 1.0],
            board_edge_radius: 3.0,
            cell_edge_color: [0.0, 0.0, 0.2, 1.0],
            cell_edge_radius: 1.0,
        }
    }
}

/// Stores visual information about a gameboard.
pub struct GameboardView {
    /// Stores gameboard view settings.
    pub settings: GameboardViewSettings,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings) -> GameboardView {
        GameboardView {
            settings: settings,
        }
    }

    fn map_color(value: u16) -> Color {
        match value {
            0    => [0.00, 0.00, 0.00, 0.0],
            1    => [0.00, 0.00, 0.00, 0.0],
            2    => [0.16, 1.00, 0.00, 1.0],
            4    => [0.33, 1.00, 0.00, 1.0],
            8    => [0.50, 1.00, 0.00, 1.0],
            16   => [0.67, 1.00, 0.00, 1.0],
            32   => [0.83, 1.00, 0.00, 1.0],
            64   => [1.00, 1.00, 0.00, 1.0],
            128  => [1.00, 0.83, 0.00, 1.0],
            256  => [1.00, 0.67, 0.00, 1.0],
            512  => [1.00, 0.50, 0.00, 1.0],
            1024 => [1.00, 0.33, 0.00, 1.0],
            2048 => [1.00, 0.16, 0.00, 1.0],
            _    => [0.00, 0.00, 0.00, 0.0],
        }
    }

    /// Draw gameboard.
    pub fn draw<G: Graphics>(&self, controller: &GameboardController, c: &Context, g: &mut G) {
        use graphics::{Line, Rectangle};
        
        let ref settings = self.settings;
        let board_rect = [
            settings.position[0], settings.position[1],
            settings.size, settings.size,
        ];
        
        // Draw board background.
        Rectangle::new(settings.board_background_color)
            .draw(board_rect, &c.draw_state, c.transform, g);
        
        let gameboardsize = controller.gameboard.cells.len();
        for x in 0..gameboardsize {
            for y in 0..gameboardsize {
                let cell_rect = [
                    x as f64 * settings.size / gameboardsize as f64 + 2.0,
                    y as f64 * settings.size / gameboardsize as f64 + 2.0,
                    settings.size / gameboardsize as f64 - 4.0,
                    settings.size / gameboardsize as f64 - 4.0,
                ];

                Rectangle::new(GameboardView::map_color(controller.gameboard.cells[x][y]))
                    .draw(cell_rect, &c.draw_state, c.transform, g);
            }
        }
        
        // Draw board edge.
//        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
//            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}
