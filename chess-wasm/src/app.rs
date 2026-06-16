use yew::prelude::*;

use crate::components::*;
use crate::state::{GameState, Mode};

#[function_component]
pub fn App() -> Html {
    let state = use_reducer(GameState::default);

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
