#![warn(missing_docs)]

//! An implementation of the popular 2048 game in Rust with Piston Game Engine

extern crate piston;
extern crate piston_window;
extern crate graphics;
extern crate opengl_graphics;

use piston::window::WindowSettings;
use piston::event_loop::{Events, EventSettings, EventLoop};
use piston::input::RenderEvent;
use piston_window::*;
use opengl_graphics::{OpenGL, GlGraphics};

pub use gameboard::Gameboard;
pub use gameboard_controller::GameboardController;
pub use gameboard_view::{GameboardView, GameboardViewSettings};
mod gameboard;
mod gameboard_controller;
mod gameboard_view;

fn main() {
    let mut window: PistonWindow = WindowSettings::new("2048", (512,512))
        .exit_on_esc(true)
        .build()
        .expect("Could not create window");

    let mut events = Events::new(EventSettings::new().lazy(true));
    //let mut gl = GlGraphics::new(opengl);

    let gameboard = Gameboard::new();
    let mut gameboard_controller = GameboardController::new(gameboard);
    let gameboard_view_settings = GameboardViewSettings::new();
    let gameboard_view = GameboardView::new(gameboard_view_settings);

    while let Some(e) = events.next(&mut window) {
        gameboard_controller.event(&e);
        if let Some(args) = e.render_args() {
            window.draw_2d(&e, |c, g| {
                use graphics::{clear};

                clear([1.0; 4], g);
                gameboard_view.draw(&gameboard_controller, &c, g);
            });
        }
    }
}
