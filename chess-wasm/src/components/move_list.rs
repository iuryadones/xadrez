use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct MoveListProps {
    pub history: Vec<String>,
}

#[function_component]
pub fn MoveList(props: &MoveListProps) -> Html {
    let moves: Vec<Html> = props.history.iter().enumerate().map(|(i, m)| {
        html! { <li key={i}>{ m }</li> }
    }).collect();

    html! {
        <div class="move-list">
            <div style="font-weight: bold; margin-bottom: 0.25rem; color: #aaa;">
                { "Lances" }
            </div>
            <ol>{ moves }</ol>
        </div>
    }
}
