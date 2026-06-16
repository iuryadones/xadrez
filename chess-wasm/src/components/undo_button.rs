use yew::prelude::*;

use crate::state::{GameAction, GameState};

#[derive(Properties, PartialEq)]
pub struct UndoButtonProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn UndoButton(props: &UndoButtonProps) -> Html {
    let onclick = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| state.dispatch(GameAction::Undo))
    };

    html! {
        <button {onclick}>{ "Desfazer" }</button>
    }
}
