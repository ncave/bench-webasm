mod NBody;
use NBody::NBody as nbody;
use wasm_bindgen::prelude::*;

use std::rc::Rc;
use fable_library_rust::*;

thread_local! {
    static BODIES: Rc<MutCell<Vec<Rc<nbody::Planet>>>> = nbody::getBodies();
}

#[wasm_bindgen]
pub fn init() {
    BODIES.with(|bodies| nbody::init(bodies))
}

#[wasm_bindgen]
pub fn step() -> f64 {
    BODIES.with(|bodies| nbody::step(bodies))
}

#[wasm_bindgen]
pub fn bench(steps: i32, dt: f64) -> f64 {
    BODIES.with(|bodies| nbody::bench(bodies, &steps, &dt))
}
