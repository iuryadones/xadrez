use chess::*;
use yew::prelude::*;

use super::square_tile::SquareTile;
use crate::state::{GameAction, GameState};

#[derive(Properties, PartialEq)]
pub struct ChessBoardProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn ChessBoard(props: &ChessBoardProps) -> Html {
    let state_handle = props.state.clone();

    let mut rows: Vec<Html> = Vec::new();

    for rank in (0..8).rev() {
        let mut squares: Vec<Html> = Vec::new();
        for file in 0..8 {
            let sq = Square::new_unchecked(file, rank);
            let is_light = (rank + file) % 2 == 0;
            let piece = state_handle.game.board().piece_at(sq);
            let is_selected = state_handle.selected == Some(sq);
            let is_legal_target = state_handle.legal_moves_for_selected.iter().any(|m| m.to == sq);
            let is_capture_target = is_legal_target && piece.is_some();

            let on_click = {
                let state = state_handle.clone();
                Callback::from(move |_| {
                    let turn = state.game.turn();
                    if state.pending_promotion.is_some() {
                        return;
                    }
                    if state.selected.is_some() && state.legal_moves_for_selected.iter().any(|m| m.to == sq) {
                        let from = state.selected.unwrap();
                        let has_promo = state.legal_moves_for_selected.iter()
                            .any(|m| m.from == from && m.to == sq && m.promotion.is_some());
                        if has_promo {
                            state.dispatch(GameAction::RequestPromotion { from, to: sq });
                            return;
                        }
                        if let Some(mv) = state.legal_moves_for_selected.iter()
                            .find(|m| m.from == from && m.to == sq) {
                            state.dispatch(GameAction::MakeMove(*mv));
                            return;
                        }
                    }
                    if state.game.board().piece_at(sq).is_some_and(|p| p.color == turn) {
                        state.dispatch(GameAction::Select(sq));
                    } else if state.selected.is_some() {
                        state.dispatch(GameAction::Deselect);
                    }
                })
            };

            squares.push(html! {
                <SquareTile {sq} {is_light} {piece} {is_selected} {is_legal_target}
                    is_capture_target={is_capture_target} on_click={on_click} />
            });
        }
        rows.push(html! { <div class="board-row">{ squares }</div> });
    }

    html! {
        <div class="board-section">
            <div class="board">
                { rows }
            </div>
        </div>
    }
}
