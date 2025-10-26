mod app;
mod models;
mod components;
mod pages;
mod services;

use app::App;
use yew::prelude::*;

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}