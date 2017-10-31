//! Gameboard view.

extern crate piston_window;

use piston_window::*;
use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;
use graphics::glyph_cache::rusttype::*;

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
        use graphics::color::hex;
        match value {
            0    => hex("00000000"),
            1    => hex("00000000"),
            2    => hex("005000FF"),
            4    => hex("55B900FF"),
            8    => hex("AADC00FF"),
            16   => hex("FFFF00FF"),
            32   => hex("FFAA00FF"),
            64   => hex("FF5500FF"),
            128  => hex("FF0000FF"),
            256  => hex("DC0032FF"),
            512  => hex("B90064FF"),
            1024 => hex("960096FF"),
            2048 => hex("000000FF"),
            _    => hex("00000000"),
        }
    }

    /// Draw gameboard.
    pub fn draw<G>(&self, controller: &GameboardController, glyphs: &mut Glyphs, c: &Context, g: &mut G)
        where G: Graphics<Texture=<Glyphs as CharacterCache>::Texture> {

        use graphics::{Rectangle};
        
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
                if controller.gameboard.cells[x][y] != 0 {
                    let size = settings.size / gameboardsize as f64 - 4.0;
                    let pos_x = x as f64 * settings.size / gameboardsize as f64 + 2.0;
                    let pos_y = y as f64 * settings.size / gameboardsize as f64 + 2.0;
    
                    Rectangle::new(GameboardView::map_color(controller.gameboard.cells[x][y]))
                        .draw(rectangle::square(0.0, 0.0, size), &c.draw_state, c.transform.trans(pos_x, pos_y), g);
    
                    // draw text
                    text::Text::new_color([1.0, 1.0, 1.0, 1.0], 32).draw(
                        &controller.gameboard.cells[x][y].to_string(),
                        glyphs,
                        &c.draw_state,
                        c.transform.trans(pos_x + 10.0, pos_y + 30.0), 
                        g
                    ).unwrap();
                }
            }
        }
        
        // Draw board edge.
//        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
//            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}
