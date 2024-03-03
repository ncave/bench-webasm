import { performance } from "perf_hooks";
import * as nbody_fs from "../pkg/fable_nbody.js";

function measureTime(f) {
    const t0 = performance.now();
    let res = f();
    const t1 = performance.now();
    return [res, t1 - t0];
}

function main() {
    const dt = 0.01;
    const steps = 50_000_000;

    console.log(`NBody steps: ${steps}, increment: ${dt}`);

    nbody_fs.init();
    nbody_fs.bench(10, dt); // warmup
    const [energy, elapsed] = measureTime(() => nbody_fs.bench(steps, dt));
    console.log(`  F# to webasm: Elapsed: ${elapsed.toFixed(3)} sec, Energy: ${energy.toFixed(17)}`);
}

main();
