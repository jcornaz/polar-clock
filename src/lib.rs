use wasm_bindgen::prelude::*;
use yew::App;

use clock::Clock;

mod svg;
mod arc;
mod vector;
mod clock;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<Clock>::new().mount_to_body();
}
