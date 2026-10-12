//! Differential runner: emits exact bit patterns, preserves observed padding.
use std::io::{self, BufRead};
use tiny_glade_reconstruction::roof_parameters::RoofParameterBlock;
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let words: Vec<_> = line.split_whitespace().collect();
        match words[0] {
            "discrete" => {
                let decode = |text: &str| -> Vec<u8> {
                    (0..text.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
                        .collect()
                };
                let mut state =
                    tiny_glade_reconstruction::roof_replay::VisualReplayState::from_observed(
                        decode(words[1]).try_into().unwrap(),
                    );
                let payload: [u8; 64] = decode(words[2]).try_into().unwrap();
                let dt = f32::from_bits(u32::from_str_radix(words[3], 16).unwrap());
                let event = state.advance_discrete_observed(&payload, dt).unwrap();
                let hex =
                    |bytes: &[u8]| bytes.iter().map(|b| format!("{b:02x}")).collect::<String>();
                if let Some(event) = event {
                    let mut record = [0xa5; 64];
                    let end = event.write_observed_fields(&mut record);
                    println!(
                        "{} {} {}",
                        hex(state.bytes()),
                        event.observed_tag(),
                        hex(&record[8..end])
                    );
                } else {
                    println!("{} - -", hex(state.bytes()));
                }
            }
            "replay" => {
                let decode = |text: &str| -> Vec<u8> {
                    (0..text.len())
                        .step_by(2)
                        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).unwrap())
                        .collect()
                };
                let mut state =
                    tiny_glade_reconstruction::roof_replay::VisualReplayState::from_observed(
                        decode(words[1]).try_into().unwrap(),
                    );
                let payload: [u8; 64] = decode(words[2]).try_into().unwrap();
                let edit =
                    tiny_glade_reconstruction::roof_replay::visual_edit_from_observed(&payload)
                        .unwrap();
                let dt = f32::from_bits(u32::from_str_radix(words[3], 16).unwrap());
                let event = state.advance(&edit, dt);
                let mut bytes = [0xa5; 64];
                tiny_glade_reconstruction::roof_replay::write_observed_event(&event, &mut bytes);
                println!(
                    "{} {}",
                    state
                        .bytes()
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>(),
                    bytes[..53]
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>()
                );
            }
            "pack" => {
                let values: Vec<f32> = words[1..7]
                    .iter()
                    .map(|s| f32::from_bits(u32::from_str_radix(s, 16).unwrap()))
                    .collect();
                let params = tg_history::RoofParameters {
                    tip_offset_01: [values[0], values[1]],
                    profile_01: values[2],
                    height_01: values[3],
                    ridge_length_01: values[4],
                    eave_length_01: values[5],
                    ridge_dir: if words[7] == "0" {
                        tg_history::RidgeDirection::Width
                    } else {
                        tg_history::RidgeDirection::Length
                    },
                    extensions: Default::default(),
                };
                let mut block = RoofParameterBlock::from_bytes([0xa5; 28]);
                block.replace_serialized_fields(&params);
                println!(
                    "{}",
                    block
                        .bytes()
                        .iter()
                        .map(|b| format!("{b:02x}"))
                        .collect::<String>()
                );
            }
            "eave" => {
                let profile = u32::from_str_radix(words[1], 16).unwrap();
                let eave = u32::from_str_radix(words[2], 16).unwrap();
                let mut raw = [0u8; 28];
                raw[8..12].copy_from_slice(&profile.to_le_bytes());
                raw[20..24].copy_from_slice(&eave.to_le_bytes());
                println!(
                    "{:08x}",
                    RoofParameterBlock::from_bytes(raw)
                        .eave_length_ws()
                        .to_bits()
                );
            }
            _ => panic!("Unknown probe mode"),
        }
    }
}
