//! Gameboard view.

extern crate piston_window;

use piston_window::*;
use graphics::types::Color;
use graphics::{Context, Graphics};
use graphics::character::CharacterCache;
use graphics::color::hex;

use GameboardController;
use TileRenderer;
use TileSettings;

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
            board_background_color: hex("BBADA0"),
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
    tile_renderer: TileRenderer,
}

impl GameboardView {
    /// Creates a new gameboard view.
    pub fn new(settings: GameboardViewSettings, glyphs: Glyphs) -> GameboardView {
        GameboardView {
            settings: settings,
            tile_renderer: TileSettings::new(80.0).with_glyphs(glyphs).build(),
        }
    }

    /// Draw gameboard.
    pub fn draw<G>(&self, controller: &GameboardController, c: &Context, g: &mut G) 
        where G: Graphics<Texture=<Glyphs as CharacterCache>::Texture>{

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
                let size = settings.size / gameboardsize as f64 - 8.0;
                let pos_x = x as f64 * settings.size / gameboardsize as f64 + 4.0;
                let pos_y = y as f64 * settings.size / gameboardsize as f64 + 4.0;

                self.tile_renderer.draw_tile(controller.gameboard.cells[x][y], &c.trans(pos_x, pos_y), g);
            }
        }
        
        // Draw board edge.
//        Rectangle::new_border(settings.board_edge_color, settings.board_edge_radius)
//            .draw(board_rect, &c.draw_state, c.transform, g);
    }
}
