#![deny(warnings)]
#![deny(clippy::all)]

mod app;
mod components;
mod render;
mod state;

fn main() {
    yew::Renderer::<app::App>::new().render();
}
