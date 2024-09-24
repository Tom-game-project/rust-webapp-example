use wasm_bindgen::prelude::*;
mod components;
use crate::components::incr::MyComponent;

#[wasm_bindgen(start)]
fn run_app() {
    yew::Renderer::<MyComponent>::new().render();
}