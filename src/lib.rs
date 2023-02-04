use wasm_bindgen::prelude::wasm_bindgen;

mod app;

use app::App;

#[wasm_bindgen]
pub fn main() {
    yew::Renderer::<App>::new().render();
}
