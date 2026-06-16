use chess::*;
use yew::prelude::*;

use crate::state::GameState;

#[derive(Properties, PartialEq)]
pub struct StatusBarProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn StatusBar(props: &StatusBarProps) -> Html {
    let game = &props.state.game;
    let status = game.status();
    let player = match game.turn() {
        Color::White => "Brancas",
        Color::Black => "Pretas",
    };

    let (text, class) = match status {
        GameStatus::Ongoing => {
            if game.in_check() {
                (format!("{} \u{26A0} XEQUE!", player), "check")
            } else {
                (format!("Vez das {}", player), "")
            }
        }
        GameStatus::WhiteWins => ("♔ XEQUE-MATE! Brancas venceram!".into(), "result"),
        GameStatus::BlackWins => ("♚ XEQUE-MATE! Pretas venceram!".into(), "result"),
        GameStatus::Draw => ("Empate!".into(), "result"),
    };

    let fullmove = game.fullmove_number();
    let halfmove = game.halfmove_clock();

    html! {
        <div class="status-bar">
            <span class={class}>{ text }</span>
            <span style="margin-left: 1rem; color: #888; font-size: 0.85rem;">
                { format!("#{}", fullmove) }
                { if halfmove > 0 { format!(" (\u{23F1}{})", halfmove) } else { String::new() } }
            </span>
        </div>
    }
}
