use chess::*;
use chess::ai::Difficulty;
use js_sys;
use yew::Reducible;

#[derive(Clone, PartialEq, Copy)]
pub enum Mode {
    PvP,
    PvBot,
}

#[derive(Clone, PartialEq)]
pub struct GameState {
    pub game: Game,
    pub selected: Option<Square>,
    pub legal_moves_for_selected: Vec<Move>,
    pub move_history: Vec<String>,
    pub input_error: bool,
    pub pending_promotion: Option<Vec<Move>>,
    pub mode: Option<Mode>,
    pub difficulty: Option<Difficulty>,
    pub bot_color: Option<Color>,
    pub bot_pending: bool,
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
    SetMode(Mode),
    SetDifficulty(Difficulty),
    BotMove(Move),
}

fn wasm_coin_flip() -> Color {
    if (js_sys::Math::random() * 2.0).floor() as u32 == 0 {
        Color::White
    } else {
        Color::Black
    }
}

fn is_bot_turn(game: &Game, mode: Option<Mode>, bot_color: Option<Color>) -> bool {
    mode == Some(Mode::PvBot) && Some(game.turn()) == bot_color
}

fn make_human_move(game: &mut Game, history: &mut Vec<String>, mv: Move) {
    let notation = move_to_algebraic(game, &mv);
    if game.make_move(mv).is_ok() {
        history.push(notation);
    }
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
            mode: None,
            difficulty: None,
            bot_color: None,
            bot_pending: false,
        }
    }
}

impl Reducible for GameState {
    type Action = GameAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        let mut next = (*self).clone();
        match action {
            GameAction::SetMode(mode) => {
                next.game = Game::new();
                next.move_history = Vec::new();
                next.mode = Some(mode);
                next.difficulty = None;
                next.bot_color = None;
                next.bot_pending = false;
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.pending_promotion = None;
                next.input_error = false;
            }
            GameAction::SetDifficulty(diff) => {
                let bot_color = wasm_coin_flip();
                next.game = Game::new();
                next.move_history = Vec::new();
                next.difficulty = Some(diff);
                next.bot_color = Some(bot_color);
                next.bot_pending = bot_color == Color::White;
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.pending_promotion = None;
                next.input_error = false;
            }
            GameAction::Select(sq) => {
                if is_bot_turn(&next.game, next.mode, next.bot_color)
                    || next.bot_pending
                {
                    return std::rc::Rc::new(next);
                }
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
                if is_bot_turn(&next.game, next.mode, next.bot_color)
                    || next.bot_pending
                {
                    return std::rc::Rc::new(next);
                }
                if mv.promotion.is_none() {
                    let candidates: Vec<Move> = next.game.legal_moves().into_iter()
                        .filter(|lm| lm.from == mv.from && lm.to == mv.to && lm.promotion.is_some())
                        .collect();
                    if !candidates.is_empty() {
                        next.pending_promotion = Some(candidates);
                        return std::rc::Rc::new(next);
                    }
                }
                make_human_move(&mut next.game, &mut next.move_history, mv);
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.input_error = false;
                if is_bot_turn(&next.game, next.mode, next.bot_color)
                    && next.game.status() == GameStatus::Ongoing
                {
                    next.bot_pending = true;
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
                make_human_move(&mut next.game, &mut next.move_history, mv);
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.input_error = false;
                if is_bot_turn(&next.game, next.mode, next.bot_color)
                    && next.game.status() == GameStatus::Ongoing
                {
                    next.bot_pending = true;
                }
            }
            GameAction::BotMove(mv) => {
                next.bot_pending = false;
                let notation = move_to_algebraic(&next.game, &mv);
                if next.game.make_move(mv).is_ok() {
                    next.move_history.push(notation);
                }
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.input_error = false;
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
