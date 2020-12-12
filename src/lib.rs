#![recursion_limit = "1024"]

use wasm_bindgen::prelude::*;
use yew::App;

use clock::PolarClock;

mod arc;
mod clock;
mod svg;
mod time;
mod vector;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<PolarClock>::new().mount_to_body();
}
