use chess::*;
use yew::prelude::*;

use crate::state::{GameAction, GameState};

#[derive(Properties, PartialEq)]
pub struct PromotionDialogProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn PromotionDialog(props: &PromotionDialogProps) -> Html {
    let candidates = match &props.state.pending_promotion {
        Some(c) => c.clone(),
        None => return html! {},
    };

    let is_light = true;

    html! {
        <div class="promotion-overlay" onclick={
            let state = props.state.clone();
            Callback::from(move |_| state.dispatch(GameAction::CancelPromotion))
        }>
            <div class="promotion-dialog" onclick={Callback::from(|e: MouseEvent| e.stop_propagation())}>
                <div class="promotion-title">{ "Promover peão" }</div>
                <div class="promotion-options">
                    { candidates.iter().map(|mv| {
                        let pt = mv.promotion.unwrap();
                        let glyph = pt.to_unicode_square(Color::White, is_light);
                        let label = pt.to_char().to_ascii_uppercase();
                        let state = props.state.clone();
                        let mv = *mv;
                        html! {
                            <button class="promotion-btn"
                                onclick={Callback::from(move |_| state.dispatch(GameAction::PromotionSelected(mv)))}>
                                <span style="font-size: 2rem;">{ glyph }</span>
                                <span class="promotion-label">{ label }</span>
                            </button>
                        }
                    }).collect::<Html>() }
                </div>
                <button class="promotion-cancel"
                    onclick={
                        let state = props.state.clone();
                        Callback::from(move |_| state.dispatch(GameAction::CancelPromotion))
                    }>
                    { "Cancelar" }
                </button>
            </div>
        </div>
    }
}
