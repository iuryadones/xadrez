use chess::*;
use yew::prelude::*;

use crate::state::{GameAction, GameState, Mode};

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
            if state.mode == Some(Mode::PvBot) && Some(state.game.turn()) == state.bot_color {
                return;
            }
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
            if state.mode == Some(Mode::PvBot) && Some(state.game.turn()) == state.bot_color {
                return;
            }
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

    let is_bot_turn = props.state.mode == Some(Mode::PvBot)
        && Some(props.state.game.turn()) == props.state.bot_color;

    html! {
        <div class="move-input-row">
            <input ref={input_ref}
                placeholder="e4, Nf3, O-O..."
                disabled={is_bot_turn}
                onkeypress={on_keypress}
            />
            <button disabled={is_bot_turn} onclick={on_submit}>{ "Jogar" }</button>
        </div>
    }
}

fn parse_input(game: &Game, input: &str) -> Option<GameAction> {
    if input.trim().is_empty() { return None; }
    parse_algebraic(game, input).map(GameAction::MakeMove)
}
