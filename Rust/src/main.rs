// #![allow(dead_code)]
// #![allow(non_snake_case)]
// #![allow(non_camel_case_types)]
// #![allow(non_upper_case_globals)]
// #![allow(unused_parens)]
// #![allow(unused_imports)]
// #![allow(unused_variables)]
// #![allow(unused_attributes)]

mod nbody;

pub fn measure_time<T>(f: impl Fn() -> T) -> (T, f64) {
    let t0 = std::time::Instant::now();
    let res: T = f();
    let t1 = std::time::Instant::now();
    (res, t1.duration_since(t0).as_secs_f64())
}

pub fn main() {
    let dt: f64 = 0.01f64;
    let steps: i32 = 20000000i32;
    nbody::init();
    nbody::bench(10i32, dt);
    let (energy, elapsed) = measure_time(|| nbody::bench(steps, dt));

    println!("Steps: {0}, increment: {1}", steps, dt);
    println!("Energy:   {0}, elapsed: {1} sec", energy, elapsed);
    println!("Expected: -0.16903264767107579");
}
