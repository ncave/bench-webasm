import * as process from "process";
import * as nbody_js from "./nbody_js";
const nbody_as = require("../assembly");
const nbody_rs = require("../rust");

function measureTime<T>(f: () => T) {
  const start = process.hrtime();
  const res = f()
  const time = process.hrtime(start);
  const elapsed = time[0] + time[1] / 1e9
  return [res, elapsed];
}

function main() {
  const dt = 0.01;
  const steps = 20_000_000;

  console.log(`Steps: ${steps}, increment: ${dt}`);

  nbody_js.init();
  nbody_js.bench(10, dt); // warmup
  const [energy1, elapsed1] = measureTime(() => nbody_js.bench(steps, dt));
  console.log(`    JavaScript: Elapsed: ${elapsed1.toFixed(3)} sec, Energy: ${energy1.toFixed(17)}`);

  nbody_as.init();
  nbody_as.bench(10, dt); // warmup
  const [energy2, elapsed2] = measureTime(() => nbody_as.bench(steps, dt));
  console.log(`AssemblyScript: Elapsed: ${elapsed2.toFixed(3)} sec, Energy: ${energy2.toFixed(17)}`);

  nbody_rs.init();
  nbody_rs.bench(10, dt); // warmup
  const [energy3, elapsed3] = measureTime(() => nbody_rs.bench(steps, dt));
  console.log(`Rust to webasm: Elapsed: ${elapsed3.toFixed(3)} sec, Energy: ${energy3.toFixed(17)}`);
}

main();
