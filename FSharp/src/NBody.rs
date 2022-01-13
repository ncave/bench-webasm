#![allow(dead_code)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_attributes)]
use std::rc::Rc;
use fable_library_rust::*;
pub mod NBody {
    use super::*;
    pub fn PI() -> f64 { 3.141592653589793f64 }
    pub fn SOLAR_MASS() -> f64 { 4.0f64 * NBody::PI() * NBody::PI() }
    pub fn YEAR() -> f64 { 365.24f64 }
    #[derive(Clone, Debug, PartialEq, PartialOrd)]
    pub struct Planet {
        pub x: MutCell<f64>,
        pub y: MutCell<f64>,
        pub z: MutCell<f64>,
        pub vx: MutCell<f64>,
        pub vy: MutCell<f64>,
        pub vz: MutCell<f64>,
        pub mass: f64,
    }
    pub fn Sun() -> Rc<NBody::Planet> {
        Rc::from(NBody::Planet{x: MutCell::from(0.0f64),
                               y: MutCell::from(0.0f64),
                               z: MutCell::from(0.0f64),
                               vx: MutCell::from(0.0f64),
                               vy: MutCell::from(0.0f64),
                               vz: MutCell::from(0.0f64),
                               mass: NBody::SOLAR_MASS(),})
    }
    pub fn Jupiter() -> Rc<NBody::Planet> {
        Rc::from(NBody::Planet{x: MutCell::from(4.841431442464721f64),
                               y: MutCell::from(-1.1603200440274284f64),
                               z: MutCell::from(-0.10362204447112311f64),
                               vx:
                                   MutCell::from(0.001660076642744037f64 *
                                                     NBody::YEAR()),
                               vy:
                                   MutCell::from(0.007699011184197404f64 *
                                                     NBody::YEAR()),
                               vz:
                                   MutCell::from(-6.90460016972063E-05f64 *
                                                     NBody::YEAR()),
                               mass:
                                   0.0009547919384243266f64 *
                                       NBody::SOLAR_MASS(),})
    }
    pub fn Saturn() -> Rc<NBody::Planet> {
        Rc::from(NBody::Planet{x: MutCell::from(8.34336671824458f64),
                               y: MutCell::from(4.124798564124305f64),
                               z: MutCell::from(-0.4035234171143214f64),
                               vx:
                                   MutCell::from(-0.002767425107268624f64 *
                                                     NBody::YEAR()),
                               vy:
                                   MutCell::from(0.004998528012349172f64 *
                                                     NBody::YEAR()),
                               vz:
                                   MutCell::from(2.3041729757376393E-05f64 *
                                                     NBody::YEAR()),
                               mass:
                                   0.0002858859806661308f64 *
                                       NBody::SOLAR_MASS(),})
    }
    pub fn Uranus() -> Rc<NBody::Planet> {
        Rc::from(NBody::Planet{x: MutCell::from(12.894369562139131f64),
                               y: MutCell::from(-15.111151401698631f64),
                               z: MutCell::from(-0.22330757889265573f64),
                               vx:
                                   MutCell::from(0.002964601375647616f64 *
                                                     NBody::YEAR()),
                               vy:
                                   MutCell::from(0.0023784717395948095f64 *
                                                     NBody::YEAR()),
                               vz:
                                   MutCell::from(-2.9658956854023756E-05f64 *
                                                     NBody::YEAR()),
                               mass:
                                   4.366244043351563E-05f64 *
                                       NBody::SOLAR_MASS(),})
    }
    pub fn Neptune() -> Rc<NBody::Planet> {
        Rc::from(NBody::Planet{x: MutCell::from(15.379697114850917f64),
                               y: MutCell::from(-25.919314609987964f64),
                               z: MutCell::from(0.17925877295037118f64),
                               vx:
                                   MutCell::from(0.0026806777249038932f64 *
                                                     NBody::YEAR()),
                               vy:
                                   MutCell::from(0.001628241700382423f64 *
                                                     NBody::YEAR()),
                               vz:
                                   MutCell::from(-9.515922545197159E-05f64 *
                                                     NBody::YEAR()),
                               mass:
                                   5.1513890204661145E-05f64 *
                                       NBody::SOLAR_MASS(),})
    }
    pub fn getBodies() -> Rc<MutCell<Vec<Rc<NBody::Planet>>>> {
        Native::arrayFrom(&[NBody::Sun(), NBody::Jupiter(), NBody::Saturn(),
                            NBody::Uranus(), NBody::Neptune()])
    }
    pub fn advance(bodies: &Rc<MutCell<Vec<Rc<NBody::Planet>>>>, dt: &f64) {
        let len: i32 = bodies.clone().len() as i32;
        for i in 0i32..=len - 1i32 {
            let bi: Rc<NBody::Planet> = bodies[i].clone();
            for j in i + 1i32..=len - 1i32 {
                let bj: Rc<NBody::Planet> = bodies[j].clone();
                let dx: f64 = bi.x.get() - bj.x.get();
                let dy: f64 = bi.y.get() - bj.y.get();
                let dz: f64 = bi.z.get() - bj.z.get();
                let d2: f64 = dx * dx + dy * dy + dz * dz;
                let mag: f64 = dt.clone() / (d2 * d2.sqrt());
                let massj_mag: f64 = bj.mass * mag;
                bi.vx.set(bi.vx.get() - dx * massj_mag);
                bi.vy.set(bi.vy.get() - dy * massj_mag);
                bi.vz.set(bi.vz.get() - dz * massj_mag);
                {
                    let massi_mag: f64 = bi.mass * mag;
                    bj.vx.set(bj.vx.get() + dx * massi_mag);
                    bj.vy.set(bj.vy.get() + dy * massi_mag);
                    bj.vz.set(bj.vz.get() + dz * massi_mag)
                }
            }
            bi.x.set(bi.x.get() + dt.clone() * bi.vx.get());
            bi.y.set(bi.y.get() + dt.clone() * bi.vy.get());
            bi.z.set(bi.z.get() + dt.clone() * bi.vz.get())
        }
    }
    pub fn energy(bodies: &Rc<MutCell<Vec<Rc<NBody::Planet>>>>) -> f64 {
        let e: Rc<MutCell<f64>> = Rc::from(MutCell::from(0.0f64));
        let len: i32 = bodies.clone().len() as i32;
        for i in 0i32..=len - 1i32 {
            let bi: Rc<NBody::Planet> = bodies[i].clone();
            e.set(e.get() +
                      (bi.vx.get() * bi.vx.get() + bi.vy.get() * bi.vy.get() +
                           bi.vz.get() * bi.vz.get()) * bi.mass / 2.0f64);
            for j in i + 1i32..=len - 1i32 {
                let bj: Rc<NBody::Planet> = bodies[j].clone();
                let dx: f64 = bi.x.get() - bj.x.get();
                let dy: f64 = bi.y.get() - bj.y.get();
                let dz: f64 = bi.z.get() - bj.z.get();
                let dist: f64 = (dx * dx + dy * dy + dz * dz).sqrt();
                e.set(e.get() - bi.mass * bj.mass / dist)
            }
        }
        e.get()
    }
    pub fn offset_momentum(bodies: &Rc<MutCell<Vec<Rc<NBody::Planet>>>>) {
        let px: Rc<MutCell<f64>> = Rc::from(MutCell::from(0.0f64));
        let py: Rc<MutCell<f64>> = Rc::from(MutCell::from(0.0f64));
        let pz: Rc<MutCell<f64>> = Rc::from(MutCell::from(0.0f64));
        for idx in 0i32..=bodies.clone().len() as i32 - 1i32 {
            let bi: Rc<NBody::Planet> = bodies[idx].clone();
            px.set(px.get() + bi.vx.get() * bi.mass);
            py.set(py.get() + bi.vy.get() * bi.mass);
            pz.set(pz.get() + bi.vz.get() * bi.mass)
        }
        {
            let sun: Rc<NBody::Planet> = bodies[0i32].clone();
            sun.vx.set(-px.get() / NBody::SOLAR_MASS());
            sun.vy.set(-py.get() / NBody::SOLAR_MASS());
            sun.vz.set(-pz.get() / NBody::SOLAR_MASS())
        }
    }
    pub fn init(bodies: &Rc<MutCell<Vec<Rc<NBody::Planet>>>>) {
        NBody::offset_momentum(bodies);
    }
    pub fn step(bodies: &Rc<MutCell<Vec<Rc<NBody::Planet>>>>) -> f64 {
        NBody::advance(bodies, &0.01f64);
        NBody::energy(bodies)
    }
    pub fn bench(bodies: &Rc<MutCell<Vec<Rc<NBody::Planet>>>>, steps: &i32,
                 dt: &f64) -> f64 {
        for _step in 0i32..=steps.clone() - 1i32 {
            NBody::advance(bodies, dt);
        }
        NBody::energy(bodies)
    }
}
