use wasm_bindgen::prelude::*;
use yew::App;

use crate::time_arc::TimeArk;

mod svg;
mod time_arc;
mod vector;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TimeArk>::new().mount_to_body();
}
