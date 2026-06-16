use chess::*;
use yew::prelude::*;

use crate::state::{GameState, GameAction};

#[derive(Properties, PartialEq)]
pub struct MoveInputProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn MoveInput(props: &MoveInputProps) -> Html {
    let input_ref = use_node_ref();

    let on_submit = {
        let state = props.state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |_: MouseEvent| {
            if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                let val = input.value();
                input.set_value("");
                if let Some(action) = parse_input(&state.game, &val) {
                    state.dispatch(action);
                }
            }
        })
    };

    let on_keypress = {
        let state = props.state.clone();
        let input_ref = input_ref.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(input) = input_ref.cast::<web_sys::HtmlInputElement>() {
                    let val = input.value();
                    input.set_value("");
                    if let Some(action) = parse_input(&state.game, &val) {
                        state.dispatch(action);
                    }
                }
            }
        })
    };

    html! {
        <div class="move-input-row">
            <input ref={input_ref}
                placeholder="e4, Nf3, O-O..."
                onkeypress={on_keypress}
            />
            <button onclick={on_submit}>{ "Jogar" }</button>
        </div>
    }
}

fn parse_input(game: &Game, input: &str) -> Option<GameAction> {
    if input.trim().is_empty() { return None; }
    parse_algebraic(game, input).map(GameAction::MakeMove)
}
