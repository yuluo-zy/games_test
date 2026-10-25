use std::io::{self, BufRead};
use tiny_glade_reconstruction::building_roof_customization::*;
fn bytes<const N: usize>(text: &str) -> [u8; N] {
    std::array::from_fn(|i| u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(bytes: &[u8]) -> String { bytes.iter().map(|x| format!("{x:02x}")).collect() }
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        let mut roof = bytes::<88>(p[1]);
        let mut tick = u32::from_str_radix(p[3], 16).unwrap();
        let now = u32::from_str_radix(p[4], 16).unwrap();
        let result = match p[0] {
            "style" => apply_flat_style_resolved(&mut roof,
                FlatRoofStyleRequest::from_event_record(&bytes::<32>(p[2])), &mut tick, now),
            "rim" => apply_flat_rim_resolved(&mut roof,
                FlatRoofRimRequest::from_event_record(&bytes::<24>(p[2])), &mut tick, now),
            _ => panic!("unknown customization"),
        };
        let (sound, pivot) = result.sound.map(|s|
            (format!("{:02x}", s.observed_kind),hex(&s.pivot.into_iter().flat_map(|f|f.to_le_bytes()).collect::<Vec<_>>())))
            .unwrap_or_else(||("-".into(),"-".into()));
        let history = result.history_mutation.map(|h|
            hex(&[vec![h.observed_tag,h.from,h.to],h.wall_id.0.to_le_bytes().to_vec()].concat()))
            .unwrap_or_else(||"-".into());
        println!("{} {:08x} {} {} {} {} {}",hex(&roof),tick,
            u8::from(result.state_changed),u8::from(result.recalculate_wall_holes.is_some()),sound,pivot,history);
    }
}
