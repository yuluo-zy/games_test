use std::io::{self, BufRead};
use tiny_glade_reconstruction::{
    building_wall_height_authority::write_resolved_source_height, building_wall_state::WallId,
};
fn decode<const N: usize>(s: &str) -> [u8; N] {
    assert_eq!(s.len(), N * 2);
    std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        let present = fields[0] == "1";
        let wall = WallId(fields[1].parse().unwrap());
        let height = f32::from_bits(u32::from_str_radix(fields[2], 16).unwrap());
        let mut source = decode::<112>(fields[3]);
        if present {
            let outcome = write_resolved_source_height(&mut source, wall, height);
            println!(
                "{} {} {} {}",
                hex(&source),
                u8::from(outcome.mark_dirty),
                outcome.wall_id.0,
                u8::from(outcome.source_written)
            );
        } else {
            println!("{} 0 - 0", hex(&source));
        }
    }
}
