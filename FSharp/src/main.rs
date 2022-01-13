#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
#[path = "./NBody.rs"]
pub(crate) mod import_74427919;
pub use import_74427919::*;
#[path = "./Platform.rs"]
pub(crate) mod import_4d9b3ddc;
pub use import_4d9b3ddc::*;
use std::rc::Rc;
use fable_library_rust::*;
pub mod TestApp {
    use super::*;
    pub fn main(_args: &Rc<MutCell<Vec<Rc<str>>>>) -> i32 {
        let bodies: Rc<MutCell<Vec<Rc<NBody::Planet>>>> = NBody::getBodies();
        let dt: f64 = 0.01f64;
        let steps: i32 = 20000000i32;
        NBody::init(&bodies);
        NBody::bench(&bodies, &10i32, &dt);
        {
            let patternInput: Rc<(f64, f64)> =
                Platform::measureTime(&Rc::from({
                                                    let bodies =
                                                        bodies.clone();
                                                    move ||
                                                        NBody::bench(&bodies,
                                                                     &steps,
                                                                     &dt)
                                                }));
            println!("Steps: {0}, increment: {1}", &steps, &dt);
            println!("Energy:   {0}, elapsed: {1} sec",
                     &patternInput.0.clone(),
                     &(patternInput.1.clone() / 1000.0f64));
            println!("Expected: -0.16903264767107579");
            0i32
        }
    }
}
pub fn main() {
    let args: Vec<String> = std::env::args().collect();
    let args: Vec<Rc<str>> = args[1..].iter().map(|s| Native::string(s)).collect();
    TestApp::main(&Native::arrayFrom(&args));
}
