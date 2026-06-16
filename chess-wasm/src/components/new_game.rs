use yew::prelude::*;

use crate::state::{GameAction, GameState};

#[derive(Properties, PartialEq)]
pub struct NewGameProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn NewGame(props: &NewGameProps) -> Html {
    let onclick = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| state.dispatch(GameAction::NewGame))
    };

    html! {
        <button {onclick}>{ "Novo Jogo" }</button>
    }
}
