use tiny_glade_reconstruction::{building_wall_collection, building_wall_state, surface};
#[path = "../../building/wall_freeform_cache.rs"]
mod wall_freeform_cache;
use std::io::{self, BufRead};
use wall_freeform_cache::*;

fn shape_probe(mut words: std::str::SplitWhitespace<'_>) {
    let mode: u32 = words.next().unwrap().parse().unwrap();
    let subtype: u8 = words.next().unwrap().parse().unwrap();
    let f = |word: &str| f32::from_bits(u32::from_str_radix(word, 16).unwrap());
    let minimum = f(words.next().unwrap());
    let maximum = f(words.next().unwrap());
    let fix = words.next().unwrap() == "1";
    let dt = f(words.next().unwrap());
    let bytes = |word: &str| {
        (0..word.len())
            .step_by(2)
            .map(|i| u8::from_str_radix(&word[i..i + 2], 16).unwrap())
            .collect::<Vec<_>>()
    };
    let mut placement: [u8; 68] = bytes(words.next().unwrap()).try_into().unwrap();
    let mut parameters: [u8; 16] = bytes(words.next().unwrap()).try_into().unwrap();
    let rect: Vec<_> = words
        .next()
        .unwrap()
        .as_bytes()
        .chunks(8)
        .map(|v| f(std::str::from_utf8(v).unwrap()))
        .collect();
    let circle: Vec<_> = words
        .next()
        .unwrap()
        .as_bytes()
        .chunks(8)
        .map(|v| f(std::str::from_utf8(v).unwrap()))
        .collect();
    let terrain = building_wall_collection::TerrainHeightGrid512 {
        encoded: std::sync::Arc::from(
            (0..512 * 512)
                .map(|i| ((i * 73 + (i / 512) * 101) & 65535) as u16)
                .collect::<Vec<_>>(),
        ),
        minimum_y: minimum,
        maximum_y: maximum,
        sampling_fix: fix,
    };
    let mut placement_tick = 0x11223344;
    let mut parameters_tick = 0x55667788;
    let outcome = calculate_shape_height(
        ShapeHeightInputs {
            app_mode_tag: mode,
            app_mode_subtype: subtype,
            rectangle: PreviewRectangleObserved {
                axes: [rect[0], rect[1]],
                center: [rect[2], rect[3]],
                dimensions: [rect[4], rect[5]],
            },
            circle: PreviewCircleObserved {
                center: [circle[0], circle[1]],
                radius: circle[2],
                angle: circle[3],
            },
            terrain: &terrain,
            delta_time: dt,
        },
        &mut placement,
        &mut parameters,
        &mut placement_tick,
        &mut parameters_tick,
        0x12345678,
    );
    let hex = |v: &[u8]| v.iter().map(|v| format!("{v:02x}")).collect::<String>();
    let scalar = |v: f32| format!("\"{:08x}\"", v.to_bits());
    let list = |v: &[f32]| {
        format!(
            "[{}]",
            v.iter().map(|&v| scalar(v)).collect::<Vec<_>>().join(",")
        )
    };
    let mut fields = vec![
        format!("\"placement\":\"{}\"", hex(&placement)),
        format!("\"parameters\":\"{}\"", hex(&parameters)),
        format!("\"placement_tick\":{placement_tick}"),
        format!("\"parameters_tick\":{parameters_tick}"),
    ];
    match outcome {
        Ok(ShapeHeightOutcome::Skipped) => fields.push("\"applied\":false".into()),
        Ok(ShapeHeightOutcome::Applied(result)) => {
            fields.push("\"applied\":true".into());
            fields.push(format!(
                "\"preview_points\":[{}]",
                result
                    .preview_curve
                    .points
                    .iter()
                    .map(|p| list(p))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            fields.push(format!(
                "\"preview_u\":{}",
                list(&result.preview_curve.points_u)
            ));
            fields.push(format!(
                "\"preview_length\":{}",
                scalar(result.preview_curve.length)
            ));
            fields.push(format!(
                "\"positions\":[{}]",
                result
                    .sampled_positions
                    .iter()
                    .map(|p| list(p))
                    .collect::<Vec<_>>()
                    .join(",")
            ));
            fields.push(format!("\"sampled_u\":{}", list(&result.sampled_u)));
            fields.push(format!(
                "\"sampled_length\":{}",
                scalar(result.sampled_length)
            ));
            fields.push(format!(
                "\"requested_extra\":{}",
                scalar(result.requested_extra)
            ));
            fields.push(format!(
                "\"target_world_height\":{}",
                scalar(result.target_world_height)
            ));
        }
        Err(error) => fields.push(format!("\"error\":\"{error:?}\"")),
    }
    println!("{{{}}}", fields.join(","));
}

fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mut words = line.split_whitespace();
        let command = words.next().unwrap();
        if command == "shape" {
            shape_probe(words);
            continue;
        }
        assert_eq!(command, "cache");
        let tag: u32 = words.next().unwrap().parse().unwrap();
        let clutter: u8 = words.next().unwrap().parse().unwrap();
        let extra = f32::from_bits(u32::from_str_radix(words.next().unwrap(), 16).unwrap());
        let segments: usize = words.next().unwrap().parse().unwrap();
        let positions: Vec<[f32; 3]> = words
            .map(|word| {
                assert_eq!(word.len(), 24);
                std::array::from_fn(|i| {
                    f32::from_bits(u32::from_str_radix(&word[i * 8..i * 8 + 8], 16).unwrap())
                })
            })
            .collect();
        let mut shape = [0; 28];
        shape[..4].copy_from_slice(&tag.to_le_bytes());
        match freeform_wall_height_cache(
            &positions,
            &building_wall_state::WallShapeSnapshot::from_observed(shape),
            extra,
            clutter,
            segments,
        ) {
            Ok(cache) => {
                let mut projection = [0xa5; 672];
                cache.write_projection(&mut projection);
                let bits =
                    |offset| u32::from_le_bytes(projection[offset..offset + 4].try_into().unwrap());
                let list = |values: &[f32]| {
                    values
                        .iter()
                        .map(|value| format!("{:08x}", value.to_bits()))
                        .collect::<Vec<_>>()
                        .join(",")
                };
                println!(
                    "{:08x} {:08x} {:08x} {:08x} {} {}",
                    bits(0x108),
                    bits(0x10c),
                    bits(0x110),
                    bits(0x114),
                    list(&cache.arch_segment_heights),
                    list(&cache.segment_heights),
                );
            }
            Err(error) => println!("error {error:?}"),
        }
    }
}
