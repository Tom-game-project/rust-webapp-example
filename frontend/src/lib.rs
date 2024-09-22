use wasm_bindgen::prelude::*;
use yew::prelude::*;

//
mod components;

use crate::components::incr::Model;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Model>::new().mount_to_body();
}