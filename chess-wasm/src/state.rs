use chess::*;
use chess::ai::{self, Difficulty};
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
        }
    }
}

fn play_bot_move(game: &mut Game, history: &mut Vec<String>, difficulty: Difficulty) {
    if let Some(mv) = ai::best_move_with_depth(game, difficulty.depth()) {
        let notation = move_to_algebraic(game, &mv);
        if game.make_move(mv).is_ok() {
            history.push(notation);
        }
    }
}

fn make_move_and_trigger_bot(
    game: &mut Game,
    history: &mut Vec<String>,
    mv: Move,
    mode: Option<Mode>,
    bot_color: Option<Color>,
    difficulty: Option<Difficulty>,
) {
    let notation = move_to_algebraic(game, &mv);
    if game.make_move(mv).is_ok() {
        history.push(notation);
        if mode == Some(Mode::PvBot)
            && game.status() == GameStatus::Ongoing
            && Some(game.turn()) == bot_color
        {
            if let Some(diff) = difficulty {
                play_bot_move(game, history, diff);
            }
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
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.pending_promotion = None;
                next.input_error = false;
            }
            GameAction::SetDifficulty(diff) => {
                let bot_color = ai::coin_flip();
                next.game = Game::new();
                next.move_history = Vec::new();
                next.difficulty = Some(diff);
                next.bot_color = Some(bot_color);
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.pending_promotion = None;
                next.input_error = false;

                if bot_color == Color::White {
                    play_bot_move(&mut next.game, &mut next.move_history, diff);
                }
            }
            GameAction::Select(sq) => {
                let turn = next.game.turn();
                if next.mode == Some(Mode::PvBot) && Some(turn) == next.bot_color {
                    next.input_error = false;
                    return std::rc::Rc::new(next);
                }
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
                if next.mode == Some(Mode::PvBot) && Some(next.game.turn()) == next.bot_color {
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
                let mode = next.mode;
                let bot_color = next.bot_color;
                let difficulty = next.difficulty;
                make_move_and_trigger_bot(
                    &mut next.game,
                    &mut next.move_history,
                    mv,
                    mode,
                    bot_color,
                    difficulty,
                );
                next.selected = None;
                next.legal_moves_for_selected = Vec::new();
                next.input_error = false;
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
                let mode = next.mode;
                let bot_color = next.bot_color;
                let difficulty = next.difficulty;
                make_move_and_trigger_bot(
                    &mut next.game,
                    &mut next.move_history,
                    mv,
                    mode,
                    bot_color,
                    difficulty,
                );
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


