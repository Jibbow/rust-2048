#![warn(missing_docs)]

//! An implementation of the popular 2048 game in Rust with Piston Game Engine

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;
extern crate find_folder;

use piston::window::WindowSettings;
use piston::event_loop::{EventLoop};
use piston::input::RenderEvent;
use piston_window::*;

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};
pub use tile_renderer::{TileSettings, TileRenderer};
mod gameboard;
mod gameboard_controller;
mod gameboard_view;
mod tile_renderer;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("2048", (400,400))
        .exit_on_esc(true)
        .resizable(false)
        .build()
        .expect("Could not create window");

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder("assets").unwrap();
    let mut glyphs = window.load_font(assets.join("FiraSans-Regular.ttf")).unwrap();

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    window.set_lazy(true);
    while let Some(e) = window.next() {
        gameboard_controller.event(&e);
        if let Some(_args) = e.render_args() {
            window.draw_2d(&e, |c, g, d| {
                clear([0.0; 4], g);

                gameboard_view.draw(&gameboard_controller, &c, g, &mut glyphs);

                glyphs.factory.encoder.flush(d);
            });
        }
    }
}
