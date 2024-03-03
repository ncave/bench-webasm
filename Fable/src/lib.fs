#if !FABLE_COMPILER_RUST
module TestLib

#else //FABLE_COMPILER_RUST
[<Fable.Core.Rust.OuterAttr("cfg", [|"target_arch = \"wasm32\""|])>]
module TestLib

open Fable.Core
open Fable.Core.Rust

let imports() =
    import "wasm_bindgen::prelude::*" ""
    ()

let bodies = NBody.getBodies()

[<OuterAttr("wasm_bindgen")>]
let init() = NBody.init(bodies)

[<OuterAttr("wasm_bindgen")>]
let step() = NBody.step(bodies)

[<OuterAttr("wasm_bindgen")>]
let bench(steps: int, dt: float) =
    NBody.bench(bodies, steps, dt)

#endif //FABLE_COMPILER_RUST
