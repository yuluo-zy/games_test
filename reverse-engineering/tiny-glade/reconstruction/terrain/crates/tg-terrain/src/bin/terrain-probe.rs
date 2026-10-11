use std::io::{self, BufRead};
use tg_terrain::{edit::{Adjustment, EditCommand, StrokeParameters, apply_resolved}, replay::{ReplayDirection, observe_edit}, schema::{StrokeId, TerrainEdit}};
fn value(text: &str) -> Result<f32, String> {
    u32::from_str_radix(text, 16).map(f32::from_bits).map_err(|e| e.to_string())
}
fn adjustment(tag: &str, number: f32) -> Result<Adjustment, String> {
    match tag { "0" => Ok(Adjustment::Keep), "1" => Ok(Adjustment::Absolute(number)), "2" => Ok(Adjustment::Delta(number)), _ => Err("invalid operation".into()) }
}
fn run(line: &str) -> Result<String, String> {
    let fields: Vec<_> = line.split_whitespace().collect();
    match fields.first().copied() {
        Some("replay") if fields.len() == 9 => {
            let direction = match fields[1] { "forward" => ReplayDirection::Forward, "backward" => ReplayDirection::Backward, _ => return Err("invalid direction".into()) };
            let index = fields[2].parse::<u32>().map_err(|e| e.to_string())?;
            let edit = TerrainEdit::EditTerrainStroke { index: StrokeId(index), prev_height_ws: value(fields[3])?, new_height_ws: value(fields[4])?, prev_falloff: value(fields[5])?, new_falloff: value(fields[6])? };
            let observed = observe_edit(&edit, value(fields[7])?, direction).map_err(|e| format!("{e:?}"))?;
            let id = fields[8].parse::<u64>().map_err(|e| e.to_string())?;
            let mut bytes = Vec::new();
            bytes.extend_from_slice(&observed.falloff_operation.to_le_bytes());
            bytes.extend_from_slice(&observed.falloff_value.to_bits().to_le_bytes());
            bytes.extend_from_slice(&observed.height_operation.to_le_bytes());
            bytes.extend_from_slice(&observed.height_value.to_bits().to_le_bytes());
            bytes.extend_from_slice(&[0; 12]);
            bytes.extend_from_slice(&observed.id.0.to_le_bytes());
            bytes.extend_from_slice(&id.to_le_bytes());
            Ok(bytes.iter().map(|b| format!("{b:02x}")).collect())
        }
        Some("edit") if fields.len() == 7 => {
            let state = StrokeParameters { falloff: value(fields[1])?, height: value(fields[2])? };
            let command = EditCommand { id: StrokeId(0), falloff: adjustment(fields[3], value(fields[4])?)?, height: adjustment(fields[5], value(fields[6])?)?, position: [0.0; 3] };
            let out = apply_resolved(state, command);
            let audio = out.audio.iter().map(|a| format!("{}:{:08x}", a.tag, a.magnitude.to_bits())).collect::<Vec<_>>().join(",");
            Ok(format!("{:08x} {:08x} {}", out.state.falloff.to_bits(), out.state.height.to_bits(), audio))
        }
        _ => Err("expected replay or edit input".into()),
    }
}
fn main() {
    for line in io::stdin().lock().lines() {
        match line.map_err(|e| e.to_string()).and_then(|line| run(&line)) {
            Ok(output) => println!("{output}"),
            Err(error) => { eprintln!("{error}"); std::process::exit(1); }
        }
    }
}
