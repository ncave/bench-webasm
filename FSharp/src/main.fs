module TestApp
open type System.Console
open NBody

[<EntryPoint>]
let main _args =
    let bodies = getBodies ()
    let dt = 0.01
    let steps = 20_000_000
    init(bodies)
    let _ = bench(bodies, 10, dt); // warmup
    let energy, elapsed = Platform.measureTime (fun () ->
        bench(bodies, steps, dt))

    WriteLine("Steps: {0}, increment: {1}", steps, dt)
    WriteLine("Energy:   {0}, elapsed: {1} sec", energy, elapsed / 1000.)
    WriteLine("Expected: -0.16903264767107579")
    0

