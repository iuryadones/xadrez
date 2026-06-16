use chess::Piece;
use yew::prelude::*;

use crate::render;

#[derive(Properties, PartialEq)]
pub struct PieceIconProps {
    pub piece: Piece,
    pub is_light: bool,
}

#[function_component]
pub fn PieceIcon(props: &PieceIconProps) -> Html {
    let glyph = render::piece_glyph(props.piece, props.is_light);
    let color = render::piece_color(props.is_light);
    html! {
        <span style={format!("color: {};", color)}>{ glyph }</span>
    }
}
