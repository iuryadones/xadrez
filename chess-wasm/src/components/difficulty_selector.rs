use chess::ai::Difficulty;
use yew::prelude::*;

use crate::state::{GameAction, GameState, Mode};

#[derive(Properties, PartialEq)]
pub struct DifficultySelectorProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn DifficultySelector(props: &DifficultySelectorProps) -> Html {
    let on_easy = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::SetDifficulty(Difficulty::Easy))
        })
    };
    let on_medium = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::SetDifficulty(Difficulty::Medium))
        })
    };
    let on_hard = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::SetDifficulty(Difficulty::Hard))
        })
    };
    let on_random = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            let idx = (js_sys::Math::random() * 3.0).floor() as u32;
            let diff = match idx {
                0 => Difficulty::Easy,
                1 => Difficulty::Medium,
                _ => Difficulty::Hard,
            };
            state.dispatch(GameAction::SetDifficulty(diff))
        })
    };
    let on_back = {
        let state = props.state.clone();
        Callback::from(move |_: MouseEvent| {
            state.dispatch(GameAction::SetMode(Mode::PvBot));
        })
    };

    html! {
        <div class="overlay">
            <div class="selector-dialog">
                <h2 class="selector-title">{ "Dificuldade do Bot" }</h2>
                <div class="selector-options">
                    <button class="selector-btn" onclick={on_easy}>
                        <span class="selector-btn-label">{ "F\u{00E1}cil" }</span>
                        <span class="selector-btn-desc">{ "Profundidade 2" }</span>
                    </button>
                    <button class="selector-btn" onclick={on_medium}>
                        <span class="selector-btn-label">{ "M\u{00E9}dio" }</span>
                        <span class="selector-btn-desc">{ "Profundidade 4" }</span>
                    </button>
                    <button class="selector-btn" onclick={on_hard}>
                        <span class="selector-btn-label">{ "Dif\u{00ED}cil" }</span>
                        <span class="selector-btn-desc">{ "Profundidade 6" }</span>
                    </button>
                    <button class="selector-btn" onclick={on_random}>
                        <span class="selector-btn-label">{ "Aleat\u{00F3}rio" }</span>
                        <span class="selector-btn-desc">{ "Sorteia F\u{00E1}cil/M\u{00E9}dio/Dif\u{00ED}cil" }</span>
                    </button>
                </div>
                <button class="selector-back" onclick={on_back}>{ "Voltar" }</button>
            </div>
        </div>
    }
}
