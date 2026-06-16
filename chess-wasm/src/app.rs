use chess::ai;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use crate::components::*;
use crate::state::{GameAction, GameState, Mode};

#[function_component]
pub fn App() -> Html {
    let state = use_reducer(GameState::default);
    let ai_scheduled = use_state(|| false);

    {
        let state = state.clone();
        let ai_scheduled = ai_scheduled.clone();
        use_effect(move || {
            if state.bot_pending && !*ai_scheduled {
                ai_scheduled.set(true);
                let state = state.clone();
                let scheduled = ai_scheduled.clone();
                let closure = Closure::once(move || {
                    if let Some(diff) = state.difficulty {
                        if let Some(mv) = ai::best_move_with_depth(&state.game, diff.depth()) {
                            state.dispatch(GameAction::BotMove(mv));
                        }
                    }
                    scheduled.set(false);
                });
                if let Some(window) = web_sys::window() {
                    window
                        .set_timeout_with_callback(closure.as_ref().unchecked_ref())
                        .ok();
                    closure.forget();
                }
            }
            || ()
        });
    }

    if state.mode.is_none() {
        return html! { <ModeSelector state={state.clone()} /> };
    }

    if state.mode == Some(Mode::PvBot) && state.difficulty.is_none() {
        return html! { <DifficultySelector state={state.clone()} /> };
    }

    let promotion_dialog = if state.pending_promotion.is_some() {
        html! { <PromotionDialog state={state.clone()} /> }
    } else {
        html! {}
    };

    html! {
        <div class="app">
            <div class="board-section">
                <StatusBar state={state.clone()} />
                <ChessBoard state={state.clone()} />
            </div>
            <div class="controls">
                <MoveInput state={state.clone()} />
                <NewGame state={state.clone()} />
                <UndoButton state={state.clone()} />
                <FenDisplay state={state.clone()} />
                <MoveList history={state.move_history.clone()} />
            </div>
            { promotion_dialog }
        </div>
    }
}
