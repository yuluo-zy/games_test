use std::io::{self, BufRead};
use tiny_glade_reconstruction::building_wall_holes::{RawWallHole, WallHoleStorage, raw_hole_hash};
fn decode(s: &str) -> RawWallHole {
    let bytes: Vec<_> = (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).unwrap())
        .collect();
    let f = |i| f32::from_le_bytes(bytes[i..i + 4].try_into().unwrap());
    RawWallHole {
        center: [f(0), f(4)],
        dimensions: [f(8), f(12)],
        kind: bytes[16],
    }
}
fn show(s: &WallHoleStorage, rebuilt: bool) -> String {
    let fp = s.fingerprint();
    let previous = s.previous_fingerprint();
    let mut out = format!(
        "{} {:08x} {} {:016x} {:016x} {:016x} {:016x} ",
        u8::from(rebuilt),
        s.wall_length().unwrap().to_bits(),
        u8::from(s.closed()),
        fp.sum,
        fp.xor,
        previous.sum,
        previous.xor
    );
    for (i, hole) in s.finalized_holes().unwrap().iter().enumerate() {
        if i != 0 {
            out.push(',');
        }
        for f in hole.aabb {
            for b in f.to_le_bytes() {
                out.push_str(&format!("{b:02x}"));
            }
        }
        out.push_str(&format!("{:02x}{:02x}", hole.kind, hole.origin));
    }
    out
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let w: Vec<_> = line.split_whitespace().collect();
        let f = |s: &str| f32::from_bits(u32::from_str_radix(s, 16).unwrap());
        if w[0] == "hash" {
            println!("{:016x}", raw_hole_hash(decode(w[1])));
            continue;
        }
        let mut s = WallHoleStorage::new_in_progress();
        if w[0] == "final" {
            for h in &w[3..] {
                s.add(decode(h)).unwrap();
            }
            match s.finalize(f(w[1]), w[2] == "1") {
                Ok(r) => println!("{}", show(&s, r)),
                Err(e) => println!("error {e:?}"),
            }
        } else {
            let initial: usize = w[5].parse().unwrap();
            for h in &w[6..6 + initial] {
                s.add(decode(h)).unwrap();
            }
            let a = s.finalize(f(w[1]), w[2] == "1").unwrap();
            let first = show(&s, a);
            let first = first.trim_end();
            let mut s = if w[0] == "frame" {
                s.advance_frame();
                s
            } else {
                s.clone_and_reopen()
            };
            for h in &w[6 + initial..] {
                s.add(decode(h)).unwrap();
            }
            match s.finalize(f(w[3]), w[4] == "1") {
                Ok(b) => println!("{first} | {}", show(&s, b)),
                Err(e) => println!("error {e:?}"),
            }
        }
    }
}
