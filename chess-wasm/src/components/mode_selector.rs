use yew::prelude::*;

use crate::state::{GameAction, GameState, Mode};

#[derive(Properties, PartialEq)]
pub struct ModeSelectorProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn ModeSelector(props: &ModeSelectorProps) -> Html {
    let on_pvp = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| state.dispatch(GameAction::SetMode(Mode::PvP)))
    };
    let on_pvbot = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| state.dispatch(GameAction::SetMode(Mode::PvBot)))
    };

    html! {
        <div class="overlay">
            <div class="selector-dialog">
                <h2 class="selector-title">{ "\u{2654} XADREZ RUST \u{265A}" }</h2>
                <p class="selector-subtitle">{ "Modo de Jogo" }</p>
                <div class="selector-options">
                    <button class="selector-btn" onclick={on_pvp}>
                        <span class="selector-btn-icon">{ "\u{1F464}" }</span>
                        <span class="selector-btn-label">{ "Jogador vs Jogador" }</span>
                    </button>
                    <button class="selector-btn" onclick={on_pvbot}>
                        <span class="selector-btn-icon">{ "\u{1F916}" }</span>
                        <span class="selector-btn-label">{ "Jogador vs Computador" }</span>
                    </button>
                </div>
            </div>
        </div>
    }
}
