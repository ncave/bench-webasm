mod nbody;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init() {
  nbody::init()
}

#[wasm_bindgen]
pub fn step() -> f64 {
  nbody::step()
}

#[wasm_bindgen]
pub fn bench(steps: i32, dt: f64) -> f64 {
  nbody::bench(steps, dt)
}
