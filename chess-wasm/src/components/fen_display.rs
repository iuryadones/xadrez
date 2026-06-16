use yew::prelude::*;

use crate::state::GameState;

#[derive(Properties, PartialEq)]
pub struct FenDisplayProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn FenDisplay(props: &FenDisplayProps) -> Html {
    let fen = props.state.game.to_fen();

    let on_copy = {
        let fen = fen.clone();
        Callback::from(move |_: MouseEvent| {
            let window = web_sys::window().unwrap();
            let _ = window.navigator().clipboard().write_text(&fen);
        })
    };

    html! {
        <div class="fen-display">
            <span>{ &fen }</span>
            <button onclick={on_copy}>{ "Copiar" }</button>
        </div>
    }
}
