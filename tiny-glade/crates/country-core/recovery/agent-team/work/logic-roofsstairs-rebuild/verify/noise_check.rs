//! Standalone verification of the rebuilt roof_noise against machine references.
//! Mirrors src/resources/roofs/roof_noise.rs exactly.

const CORNER_SEED: u32 = 0x0ae135d3;
const SCALAR_CORNER_SEED: u32 = 0x2219_927c;

fn hash1(v: u32) -> u32 {
    let a = v.wrapping_mul(0x401);
    let h = (a >> 6) ^ a;
    h.wrapping_mul(9)
}

fn hash_combine2(h: u32, y: u32) -> u32 {
    let h = (h >> 0xb ^ h).wrapping_mul(0x8001);
    let h = y.wrapping_mul(0x19660d).wrapping_add(h).wrapping_add(0x19660d);
    h.wrapping_mul(0x19660d).wrapping_add(0xae135d3)
}

fn combine_partial(h: u32, x: u32) -> u32 {
    let h = (h >> 0xb ^ h).wrapping_mul(0x8001);
    h.wrapping_add(x.wrapping_mul(0x19660d)).wrapping_mul(0x19660d)
}

fn finalize_hash(h: u32) -> u32 {
    let h = (h >> 0xb) ^ h;
    let h = (h << 7 & 0x9d2c5680) ^ h;
    let h = (h << 0xf & 0xefc60000) ^ h;
    (h & 0x7fffff) ^ (h >> 0x12)
}

fn hash2(x: u32, z: u32, seed: u32) -> u32 {
    finalize_hash(combine_partial(hash1(z), x).wrapping_add(seed))
}

fn uint_to_u01_float(bits: u32) -> f32 {
    f32::from_bits((bits & 0x007f_ffff) | 0x3f80_0000) - 1.0
}

fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

fn noise(x: f32, z: f32) -> f32 {
    let x0 = x.floor();
    let z0 = z.floor();
    let fx = x - x0;
    let fz = z - z0;
    let x0 = x0 as i32 as u32;
    let z0 = z0 as i32 as u32;
    let n00 = uint_to_u01_float(hash2(x0, z0, SCALAR_CORNER_SEED));
    let n10 = uint_to_u01_float(hash2(x0, z0, CORNER_SEED));
    let n01 = uint_to_u01_float(hash2(x0, z0.wrapping_add(1), SCALAR_CORNER_SEED));
    let n11 = uint_to_u01_float(hash2(x0, z0.wrapping_add(1), CORNER_SEED));
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sz = fz * fz * (3.0 - 2.0 * fz);
    let nx = lerp(n10, n00, sx);
    let ny = lerp(n11, n01, sx);
    lerp(nx, ny, sz)
}

fn main() {
    // Machine references (f64 model of the same formula).
    let refs: &[(f32, f32, f32)] = &[
        (0.0, 0.0, 0.5487866),
        (0.5, 0.5, 0.4374025),
        (0.0, 0.5, 0.5297968),
        (0.5, 0.0, 0.4771236),
        (1.0, 1.0, 0.2845558),
        (10.0, -3.0, 0.5514752),
        (0.1, 0.2, 0.5405821),
        (-0.5, -0.5, 0.5836071),
        (123.4, 567.8, 0.4278883),
    ];
    let mut failures = 0;
    for (x, z, expected) in refs {
        let n = noise(*x, *z);
        let ok = (n - *expected).abs() < 2e-6;
        if !ok { failures += 1; }
        println!("noise({x},{z}) = {n}  expected ~{expected}  {}", if ok { "OK" } else { "FAIL" });
    }
    // hash2 corner u01 check.
    let u = uint_to_u01_float(hash2(0, 0, CORNER_SEED));
    println!("hash2(0,0,seed) u01 = {u}  expected ~0.5487866  {}", if (u - 0.5487866).abs() < 2e-6 { "OK" } else { "FAIL" });
    let scalar = finalize_hash(combine_partial(hash1(0), 0).wrapping_add(SCALAR_CORNER_SEED));
    let combine2 = hash_combine2(hash1(0), 0);
    println!("scalar == finalize(combine2)? {}", if scalar == finalize_hash(combine2) { "OK" } else { "FAIL" });
    println!("failures = {failures}");
}
