use game::Game;
use wasm_bindgen::prelude::*;

pub mod entities;
pub mod game;
pub mod js;
pub mod plugins;

#[wasm_bindgen]
pub fn attach_canvas(canvas_id: String) {
    Game::new(canvas_id).run();
}

#[wasm_bindgen(start)]
pub fn main() {
    js::console::log("vtt package...loaded".to_owned())
}
