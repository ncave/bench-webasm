mod nbody;

pub fn measure_time<T>(f: impl Fn() -> T) -> (T, f64) {
    let t0 = std::time::Instant::now();
    let res: T = f();
    let t1 = std::time::Instant::now();
    (res, t1.duration_since(t0).as_secs_f64())
}

pub fn main() {
    let dt: f64 = 0.01_f64;
    let steps: i32 = 50_000_000_i32;
    let energy0 = nbody::init();
    // nbody::bench(10i32, dt); // warmup
    let (energy, elapsed) = measure_time(|| nbody::bench(steps, dt));

    println!("Steps: {0}, increment: {1}", steps, dt);
    println!("Start Energy: {0}", energy0);
    println!("Final Energy: {0}, elapsed: {1} sec", energy, elapsed);
    // println!("    Expected: -0.16903264767107579"); //20m
    println!("    Expected: -0.16905990681396785"); //50m
}
