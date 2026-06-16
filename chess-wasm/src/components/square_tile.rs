use chess::*;
use yew::prelude::*;

use super::piece_icon::PieceIcon;

#[derive(Properties, PartialEq)]
pub struct SquareTileProps {
    pub sq: Square,
    pub is_light: bool,
    pub piece: Option<Piece>,
    pub is_selected: bool,
    pub is_legal_target: bool,
    pub is_capture_target: bool,
    pub on_click: Callback<()>,
}

#[function_component]
pub fn SquareTile(props: &SquareTileProps) -> Html {
    let mut classes = vec!["square"];
    if props.is_light { classes.push("light"); }
    else { classes.push("dark"); }
    if props.is_selected { classes.push("selected"); }
    if props.is_legal_target {
        classes.push("legal-target");
        if props.is_capture_target { classes.push("capture"); }
    }

    let rank_label = if props.sq.file == 0 {
        html! { <span class="coord-rank">{ props.sq.rank + 1 }</span> }
    } else {
        html! {}
    };

    let file_label = if props.sq.rank == 0 {
        let label = (b'a' + props.sq.file as u8) as char;
        html! { <span class="coord-file">{ label }</span> }
    } else {
        html! {}
    };

    let onclick = {
        let cb = props.on_click.clone();
        Callback::from(move |_| cb.emit(()))
    };

    html! {
        <div class={classes.join(" ")} {onclick}>
            { rank_label }
            { props.piece.map(|p| html! { <PieceIcon piece={p} is_light={props.is_light} /> }) }
            { file_label }
        </div>
    }
}
