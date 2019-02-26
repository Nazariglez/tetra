//! Common color definitions.

use crate::graphics::Color;

/// Shortcut for Color::rgb(0.0, 0.0, 0.0).
pub const BLACK: Color = Color {
    r: 0.0,
    g: 0.0,
    b: 0.0,
    a: 1.0,
};

/// Shortcut for Color::rgb(1.0, 1.0, 1.0).
pub const WHITE: Color = Color {
    r: 1.0,
    g: 1.0,
    b: 1.0,
    a: 1.0,
};
