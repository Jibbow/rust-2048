//! TileRenderer for rendering single tiles on the screen.

use piston_window::*;
use graphics::color::hex;
use graphics::{Context, Graphics};
use graphics::types::Color;
use graphics::character::CharacterCache;
use std::path::Path;
use std::cell::RefCell;
use std::ops::DerefMut;


/// Mapping between tile score and colors.
/// The returned tuple contains two colors:
///  - the first one is the background color for the tile
///  - the second one is the font color fot the tile
///
///  * `value`: Value of the tile. Different values result in
///             different colors.
fn map_color(value: u16) -> (Color, Color) {
    match value {
        0    => (hex("CCC0B3FF"),hex("00000000")),
        1    => (hex("00000000"),hex("00000000")),
        2    => (hex("EEE4DAFF"),hex("776E65FF")),
        4    => (hex("EDE0C8FF"),hex("776E65FF")),
        8    => (hex("F2B179FF"),hex("F9F6F2FF")),
        16   => (hex("F59563FF"),hex("F9F6F2FF")),
        32   => (hex("F67C5FFF"),hex("F9F6F2FF")),
        64   => (hex("FF5500FF"),hex("F9F6F2FF")),
        128  => (hex("EDCF72FF"),hex("F9F6F2FF")),
        256  => (hex("EDCC61FF"),hex("F9F6F2FF")),
        512  => (hex("EDC850FF"),hex("F9F6F2FF")),
        1024 => (hex("EDC53FFF"),hex("F9F6F2FF")),
        2048 => (hex("EDC22EFF"),hex("F9F6F2FF")),
        _    => (hex("3E3933FF"),hex("F9F6F2FF")),
    }
}

/// [Builder Pattern]
/// Builds a new TileRenderer based on the given settings.
pub struct TileSettings {
    size: f64,
    // glyph cache
    glyphs: Option<RefCell<Glyphs>>,
    color_mapping: fn(u16) -> (Color, Color),
}

impl TileSettings {

    /// Creates a new instance of `TileSettings`.
    /// Use the `build()` method to create a new `TileRenderer` from these settings.
    ///
    /// * `size`: Size of a rendered tile (width and height)
    pub fn new(size: f64) -> Self {
        TileSettings {
            size: size,
            color_mapping: map_color,
            glyphs: None,
        }
    }

    /// Sets the glyph cache for the `TileRenderer` based on an existing cache.
    /// Moves the reference to this `TileRenderer` and can therefore not be shared.  
    ///
    /// `Glyphs` are usually created with `Glyphs::new(font, factory,
    /// TextureSettings::new()).unwrap()` with `PistonWindow`.
    pub fn with_glyphs(mut self, glyphs: Glyphs) -> TileSettings {
        self.glyphs = Some(RefCell::new(glyphs));
        self
    }

    /// Sets the glyph cache for the TileRenderer based on an existing cache.
    /// Allows a shared cache between multiple renderers.
    pub fn with_ref_glyphs(mut self, glyphs: RefCell<Glyphs>) -> TileSettings {
        self.glyphs = Some(glyphs);
        self
    }

    /// Creates a new glyph cache based on a given font file.
    /// * `path`:    Path to the font file.
    /// * `factory`: GfxFactory for creating resources.  
    ///            Usually created with `window.factory.clone();` where `window`
    ///            is a `PistonWindow`.
    pub fn with_font(mut self, path: &Path, factory: GfxFactory) -> TileSettings {
        self.glyphs = Some(RefCell::new(Glyphs::new(path, factory, TextureSettings::new()).unwrap()));
        self
    }

    /// Constructs a TileRenderer based on the given settings.
    ///
    /// Panics, if no `GlyphCache` has been passed via the builder methods.
    pub fn build(self) -> TileRenderer {
        TileRenderer {
            size: self.size,
            glyphs: self.glyphs.expect("No GlyphCache given!"),
            color_mapping: self.color_mapping,
        }
    }
}


/// Actual tile renderer
pub struct TileRenderer {
    /// The size of a single tile
    size: f64,
    /// The glyph cache for font
    glyphs: RefCell<Glyphs>,
    /// A function that determines the drawn colors
    color_mapping: fn(u16) -> (Color, Color),
}

impl TileRenderer {
    /// Draws a tile from the game board on the screen.
    ///
    /// Use the transformation matrix of c for positioning and rotation.
    ///
    /// * `tile_value`: Number that is displayed on the tile. This value  
    ///                 determines the color of the rendered tile.
    /// * `c`:          Graphics context for drawing.
    /// * `g`:          Graphics module for drawing.
    pub fn draw_tile<G>(&self, tile_value: u16, c: &Context, g: &mut G)
        where G: Graphics<Texture = <Glyphs as CharacterCache>::Texture> {

        // draw tile
        Rectangle::new_round((self.color_mapping)(tile_value).0, 5.0)
            .draw(rectangle::square(0.0, 0.0, self.size), &c.draw_state, c.transform, g);
        
        // draw text
        text::Text::new_color((self.color_mapping)(tile_value).1, 32).draw(
            &tile_value.to_string(),
            self.glyphs.borrow_mut().deref_mut(),
            &c.draw_state,
            c.transform.trans(10.0, 30.0), 
            g
        ).unwrap();
    }
}