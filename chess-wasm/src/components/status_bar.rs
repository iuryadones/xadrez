use chess::*;
use yew::prelude::*;

use crate::state::{GameState, Mode};

#[derive(Properties, PartialEq)]
pub struct StatusBarProps {
    pub state: yew::UseReducerHandle<GameState>,
}

#[function_component]
pub fn StatusBar(props: &StatusBarProps) -> Html {
    let game = &props.state.game;
    let status = game.status();

    if props.state.bot_pending {
        return html! {
            <div class="status-bar">
                <span style="color: #7fc97f; font-weight: bold;">{ "\u{1F916} Pensando..." }</span>
            </div>
        };
    }

    let player = match game.turn() {
        Color::White => "Brancas",
        Color::Black => "Pretas",
    };

    let (text, class) = match status {
        GameStatus::Ongoing => {
            if props.state.mode == Some(Mode::PvBot) {
                let you = if game.turn() == props.state.bot_color.unwrap() {
                    "Bot".to_string()
                } else {
                    "Voc\u{00EA}".to_string()
                };
                if game.in_check() {
                    (format!("{} \u{26A0} XEQUE!", player), "check")
                } else {
                    (format!("Vez de {} ({})", player, you), "")
                }
            } else if game.in_check() {
                (format!("{} \u{26A0} XEQUE!", player), "check")
            } else {
                (format!("Vez das {}", player), "")
            }
        }
        GameStatus::WhiteWins => ("\u{2654} XEQUE-MATE! Brancas venceram!".into(), "result"),
        GameStatus::BlackWins => ("\u{265A} XEQUE-MATE! Pretas venceram!".into(), "result"),
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
