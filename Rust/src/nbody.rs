// Code adopted from https://benchmarksgame-team.pages.debian.net/benchmarksgame/program/nbody-rust-1.html

#![allow(dead_code)]
#![allow(non_upper_case_globals)]

use std::cell::Cell;

const PI: f64 = 3.141592653589793;
const SOLAR_MASS: f64 = 4.0 * PI * PI;
const YEAR: f64 = 365.24;
const N_BODIES: usize = 5;

#[derive(Clone)]
struct Planet {
  x: Cell<f64>,
  y: Cell<f64>,
  z: Cell<f64>,
  vx: Cell<f64>,
  vy: Cell<f64>,
  vz: Cell<f64>,
  mass: Cell<f64>,
}

const Sun: Planet = Planet {
  x: Cell::new(0.0),
  y: Cell::new(0.0),
  z: Cell::new(0.0),
  vx: Cell::new(0.0),
  vy: Cell::new(0.0),
  vz: Cell::new(0.0),
  mass: Cell::new(SOLAR_MASS),
};

const Jupiter: Planet = Planet {
  x: Cell::new(4.84143144246472090e+00),
  y: Cell::new(-1.16032004402742839e+00),
  z: Cell::new(-1.03622044471123109e-01),
  vx: Cell::new(1.66007664274403694e-03 * YEAR),
  vy: Cell::new(7.69901118419740425e-03 * YEAR),
  vz: Cell::new(-6.90460016972063023e-05 * YEAR),
  mass: Cell::new(9.54791938424326609e-04 * SOLAR_MASS),
};

const Saturn: Planet = Planet {
  x: Cell::new(8.34336671824457987e+00),
  y: Cell::new(4.12479856412430479e+00),
  z: Cell::new(-4.03523417114321381e-01),
  vx: Cell::new(-2.76742510726862411e-03 * YEAR),
  vy: Cell::new(4.99852801234917238e-03 * YEAR),
  vz: Cell::new(2.30417297573763929e-05 * YEAR),
  mass: Cell::new(2.85885980666130812e-04 * SOLAR_MASS),
};

const Uranus: Planet = Planet {
  x: Cell::new(1.28943695621391310e+01),
  y: Cell::new(-1.51111514016986312e+01),
  z: Cell::new(-2.23307578892655734e-01),
  vx: Cell::new(2.96460137564761618e-03 * YEAR),
  vy: Cell::new(2.37847173959480950e-03 * YEAR),
  vz: Cell::new(-2.96589568540237556e-05 * YEAR),
  mass: Cell::new(4.36624404335156298e-05 * SOLAR_MASS),
};

const Neptune: Planet = Planet {
  x: Cell::new(1.53796971148509165e+01),
  y: Cell::new(-2.59193146099879641e+01),
  z: Cell::new(1.79258772950371181e-01),
  vx: Cell::new(2.68067772490389322e-03 * YEAR),
  vy: Cell::new(1.62824170038242295e-03 * YEAR),
  vz: Cell::new(-9.51592254519715870e-05 * YEAR),
  mass: Cell::new(5.15138902046611451e-05 * SOLAR_MASS),
};

fn advance(bodies: &[Planet], dt: f64) {
  for i in 0..N_BODIES {
    let bi = &bodies[i];
    for j in (i + 1)..N_BODIES {
      let bj = &bodies[j];

      let dx = bi.x.get() - bj.x.get();
      let dy = bi.y.get() - bj.y.get();
      let dz = bi.z.get() - bj.z.get();

      let d2 = dx * dx + dy * dy + dz * dz;
      let mag = dt / (d2 * d2.sqrt());

      let massj_mag = bj.mass.get() * mag;
      bi.vx.set(bi.vx.get() - dx * massj_mag);
      bi.vy.set(bi.vy.get() - dy * massj_mag);
      bi.vz.set(bi.vz.get() - dz * massj_mag);

      let massi_mag = bi.mass.get() * mag;
      bj.vx.set(bj.vx.get() + dx * massi_mag);
      bj.vy.set(bj.vy.get() + dy * massi_mag);
      bj.vz.set(bj.vz.get() + dz * massi_mag);
    }
    bi.x.set(bi.x.get() + dt * bi.vx.get());
    bi.y.set(bi.y.get() + dt * bi.vy.get());
    bi.z.set(bi.z.get() + dt * bi.vz.get());
  }
}

fn energy(bodies: &[Planet]) -> f64 {
  let mut e = 0.0;
  for i in 0..N_BODIES {
    let bi = &bodies[i];
    e += (bi.vx.get() * bi.vx.get() + bi.vy.get() * bi.vy.get() + bi.vz.get() * bi.vz.get()) * bi.mass.get() / 2.0;
    for j in (i + 1)..N_BODIES {
      let bj = &bodies[j];
      let dx = bi.x.get() - bj.x.get();
      let dy = bi.y.get() - bj.y.get();
      let dz = bi.z.get() - bj.z.get();
      let dist = (dx * dx + dy * dy + dz * dz).sqrt();
      e -= bi.mass.get() * bj.mass.get() / dist;
    }
  }
  e
}

fn offset_momentum(bodies: &[Planet]) {
  let mut px = 0.0;
  let mut py = 0.0;
  let mut pz = 0.0;
  for bi in bodies.iter() {
    px += bi.vx.get() * bi.mass.get();
    py += bi.vy.get() * bi.mass.get();
    pz += bi.vz .get()* bi.mass.get();
  }
  let sun = &bodies[0];
  sun.vx.set(-px / SOLAR_MASS);
  sun.vy.set(-py / SOLAR_MASS);
  sun.vz.set(-pz / SOLAR_MASS);
}

thread_local! {
  static BODIES: [Planet; N_BODIES] = [Sun, Jupiter, Saturn, Uranus, Neptune];
}

pub fn init() {
  BODIES.with(|bodies| {
    offset_momentum(bodies)
  })
}

pub fn step() -> f64 {
  BODIES.with(|bodies| {
    advance(bodies, 0.01);
    energy(bodies)
  })
}

pub fn bench(steps: i32, dt: f64) -> f64 {
  BODIES.with(|bodies| {
    for _ in 0..steps {
      advance(bodies, dt);
    }
    energy(bodies)
  })
}
