use game::Game;
use wasm_bindgen::prelude::*;

pub mod game;
pub mod plugins;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn console_log(s: &str);
}

#[wasm_bindgen(start)]
pub fn min() {
    console_log("vtt package...loaded")
}

#[wasm_bindgen]
pub fn attach_canvas(canvas_id: String) {
    Game::new(canvas_id).run();
}
