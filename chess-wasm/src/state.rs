use chess::*;
use yew::Reducible;

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub game: Game,
    pub selected: Option<Square>,
    pub legal_moves_for_selected: Vec<Move>,
    pub move_history: Vec<String>,
    pub input_error: bool,
    pub pending_promotion: Option<Vec<Move>>,
}

pub enum GameAction {
    Select(Square),
    Deselect,
    MakeMove(Move),
    RequestPromotion { from: Square, to: Square },
    NewGame,
    Undo,
    PromotionSelected(Move),
    CancelPromotion,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            game: Game::new(),
            selected: None,
            legal_moves_for_selected: Vec::new(),
            move_history: Vec::new(),
            input_error: false,
            pending_promotion: None,
        }
    }
}

impl Reducible for GameState {
    type Action = GameAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut next = (*self).clone();
        match action {
            GameAction::Select(sq) => {
                let turn = next.game.turn();
                if next.game.board().piece_at(sq).is_some_and(|p| p.color == turn) {
                    next.selected = Some(sq);
                    next.legal_moves_for_selected = next.game.legal_moves().into_iter()
                        .filter(|m| m.from == sq)
                        .collect();
                } else {
                    next.selected = None;
                    next.legal_moves_for_selected = Vec::new();
                }
                next.input_error = false;
            }
            GameAction::Deselect => {
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.pending_promotion = None;
                next.input_error = false;
            }
            GameAction::MakeMove(mv) => {
                if mv.promotion.is_none() {
                    let candidates: Vec<Move> = next.game.legal_moves().into_iter()
                        .filter(|lm| lm.from == mv.from && lm.to == mv.to && lm.promotion.is_some())
                        .collect();
                    if !candidates.is_empty() {
                        next.pending_promotion = Some(candidates);
                        return std::rc::Rc::new(next);
                    }
                }
                let notation = move_to_algebraic(&next.game, &mv);
                if next.game.make_move(mv).is_ok() {
                    next.move_history.push(notation);
                    next.selected = None;
                    next.legal_moves_for_selected = Vec::new();
                    next.input_error = false;
                } else {
                    next.input_error = true;
                }
            }
            GameAction::RequestPromotion { from, to } => {
                let candidates: Vec<Move> = next.game.legal_moves().into_iter()
                    .filter(|lm| lm.from == from && lm.to == to && lm.promotion.is_some())
                    .collect();
                if !candidates.is_empty() {
                    next.pending_promotion = Some(candidates);
                }
            }
            GameAction::PromotionSelected(mv) => {
                next.pending_promotion = None;
                let notation = move_to_algebraic(&next.game, &mv);
                if next.game.make_move(mv).is_ok() {
                    next.move_history.push(notation);
                    next.selected = None;
                    next.legal_moves_for_selected = Vec::new();
                    next.input_error = false;
                }
            }
            GameAction::NewGame => {
                return std::rc::Rc::new(GameState::default());
            }
            GameAction::Undo => {
                if next.game.undo() {
                    next.move_history.pop();
                    next.selected = None;
                    next.legal_moves_for_selected = Vec::new();
                    next.input_error = false;
                }
            }
            GameAction::CancelPromotion => {
                next.pending_promotion = None;
            }
        }
        std::rc::Rc::new(next)
    }
}


