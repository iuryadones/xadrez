use chess::*;

pub const FG_DARK: &str = "#000000";
pub const FG_LIGHT: &str = "#ffffff";

pub fn piece_color(is_light: bool) -> &'static str {
    if is_light { FG_DARK } else { FG_LIGHT }
}

pub fn piece_glyph(piece: Piece, is_light: bool) -> &'static str {
    piece.kind.to_unicode_square(piece.color, is_light)
}
