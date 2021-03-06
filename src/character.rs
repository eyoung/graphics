//! A text character

use internal;
use ImageSize;

/// Holds rendered character data.
pub struct Character<T: ImageSize> {
    /// The offset of character.
    pub offset: [f64; 2],
    /// The size of character, including space.
    pub size: [f64; 2],
    /// The texture of the character.
    pub texture: T,
}

impl<T: ImageSize> Character<T> {
    /// The left offset.
    pub fn left(&self) -> f64 {
        self.offset[0]
    }

    /// The top offset.
    pub fn top(&self) -> f64 {
        self.offset[1]
    }

    /// Gets width of character, including space to the next one.
    pub fn width(&self) -> f64 {
        self.size[0]
    }

    /// Sets height of character, including space to the next one.
    pub fn height(&self) -> f64 {
        self.size[1]
    }
}

/// Stores characters in a buffer and loads them by demand.
pub trait CharacterCache<T: ImageSize> {
    /// Get reference to character.
    fn character(
        &mut self, 
        font_size: internal::FontSize, 
        ch: char
    ) -> &Character<T>;
}

