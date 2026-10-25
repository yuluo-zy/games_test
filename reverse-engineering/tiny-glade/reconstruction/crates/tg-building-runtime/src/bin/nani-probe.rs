use std::io::{self, BufRead};
use tg_building_runtime::nani::project_roof_stone_floor;
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let fields: Vec<_> = line.split_whitespace().collect();
        let mut roof = [0; 88];
        for (i, byte) in roof.iter_mut().enumerate() {
            *byte = u8::from_str_radix(&fields[0][i * 2..i * 2 + 2], 16).unwrap();
        }
        let animation = f32::from_bits(u32::from_str_radix(fields[1], 16).unwrap());
        let result = project_roof_stone_floor(&roof, animation).unwrap();
        let record: String = result.record.iter().map(|x| format!("{x:02x}")).collect();
        let bounds: String = result
            .bounds_center
            .into_iter()
            .chain([result.bounds_radius])
            .flat_map(|x| x.to_le_bytes())
            .map(|x| format!("{x:02x}"))
            .collect();
        println!("{record} {bounds}");
    }
}
