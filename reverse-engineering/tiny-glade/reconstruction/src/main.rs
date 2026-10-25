use tiny_glade_reconstruction::{
    SOURCE_SHA256, ridge, roof_pipeline, roof_state, surface, window_aspect,
};

fn roof_bytes(s: &str) -> [u8; 88] {
    assert_eq!(s.len(), 176, "Roof input must contain 88 bytes as hex");
    std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).expect("invalid hex"))
}
fn point_bits(p: [f32; 3]) -> String {
    format!(
        "[\"{:08x}\",\"{:08x}\",\"{:08x}\"]",
        p[0].to_bits(),
        p[1].to_bits(),
        p[2].to_bits()
    )
}

fn observed_bytes<const N: usize>(text: &str) -> Result<[u8; N], String> {
    if text.len() != N * 2 || !text.bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err(format!("Expected {N} bytes of hexadecimal input"));
    }
    Ok(std::array::from_fn(|i| {
        u8::from_str_radix(&text[i * 2..i * 2 + 2], 16).unwrap()
    }))
}
fn byte_hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}
fn building_roof_command(args: &[String]) -> Result<String, String> {
    use tiny_glade_reconstruction::{
        building_roof_creation as creation, building_roof_spatial as spatial,
        building_wall_state as wall,
    };
    let wall = wall::PublicWallBuildingState::from_observed(observed_bytes(&args[1])?);
    if args[0] == "building-roof-create" {
        let request = creation::RoofCreateRequestObserved::from_observed(observed_bytes(&args[2])?);
        let mut roof = observed_bytes(&args[3])?;
        let outcome = creation::initialize_resolved_roof(&mut roof, &request, &wall)
            .map_err(|e| format!("{e:?}"))?;
        let notification = outcome
            .gable_notification
            .map(|id| format!("\"{}\"", id.0))
            .unwrap_or("null".into());
        Ok(format!(
            "{{\"baseline\":\"B\",\"resolved_only\":true,\"entity_spawned\":false,\"roof_hex\":\"{}\",\"pitched\":{},\"gable_notification_wall_id\":{}}}",
            byte_hex(&roof),
            outcome.pitched,
            notification
        ))
    } else {
        let mut roof = observed_bytes(&args[2])?;
        let mut related = observed_bytes(&args[3])?;
        let wall_id =
            wall::WallId(u64::from_str_radix(&args[4], 16).map_err(|_| "wall ID must be hex u64")?);
        let entity = u64::from_str_radix(&args[5], 16).map_err(|_| "entity must be hex u64")?;
        let mut roof_tick = 0;
        let mut related_tick = 0;
        let outcome = spatial::synchronize_resolved_roof(
            &mut roof,
            &mut related,
            &wall,
            wall_id,
            entity,
            &mut roof_tick,
            &mut related_tick,
            1,
        )
        .map_err(|e| format!("{e:?}"))?;
        let events = outcome
            .geometry_changes
            .iter()
            .map(|e| {
                format!(
                    "{{\"wall_id\":\"{}\",\"roof_entity\":\"{}\",\"kind\":{}}}",
                    e.wall_id.0, e.roof_entity, e.observed_kind
                )
            })
            .collect::<Vec<_>>()
            .join(",");
        Ok(format!(
            "{{\"baseline\":\"B\",\"resolved_only\":true,\"roof_hex\":\"{}\",\"related_hex\":\"{}\",\"roof_tick\":{},\"related_tick\":{},\"style_changed\":{},\"shape_changed\":{},\"geometry_changes\":[{}]}}",
            byte_hex(&roof),
            byte_hex(&related),
            roof_tick,
            related_tick,
            outcome.style_changed,
            outcome.shape_changed,
            events
        ))
    }
}

fn roof_history_step(args: &[String]) -> Result<String, String> {
    let bytes = std::fs::read(&args[1]).map_err(|e| e.to_string())?;
    let document = tg_history::HistoryDocument::parse(&bytes).map_err(|e| e.to_string())?;
    let entry: usize = args[2].parse().map_err(|_| "entry must be usize")?;
    let edit: usize = args[3].parse().map_err(|_| "edit must be usize")?;
    let command = document
        .roof_commands()
        .map_err(|e| e.to_string())?
        .into_iter()
        .find(|c| c.entry_index == entry && c.edit_index == edit)
        .ok_or("No known Roof command at the requested entry/edit")?;
    let tg_history::RoofCommand::VisualEdit(command) = command.command else {
        return Err("This resolved runner currently supports Roof::VisualEdit".into());
    };
    if args[4].len() != 176 || !args[4].bytes().all(|b| b.is_ascii_hexdigit()) {
        return Err("Roof input must contain 88 bytes as hex".into());
    }
    let mut roof = roof_bytes(&args[4]);
    if u32::from_le_bytes(roof[8..12].try_into().unwrap()) > 1 {
        return Err("Roof shape tag is outside the recovered pivot branches".into());
    }
    let duration: f32 = args[5].parse().map_err(|_| "duration must be f32")?;
    let elapsed: f32 = args[6].parse().map_err(|_| "elapsed must be f32")?;
    let delta: f32 = args[7].parse().map_err(|_| "delta must be f32")?;
    let undo = match args[8].as_str() {
        "0" => 0,
        "1" => 1,
        _ => return Err("undo must be 0 or 1".into()),
    };
    let mut raw_state = [0; 24];
    raw_state[..4].copy_from_slice(&1u32.to_le_bytes());
    raw_state[4..8].copy_from_slice(&duration.to_le_bytes());
    raw_state[16..20].copy_from_slice(&elapsed.to_le_bytes());
    raw_state[20] = undo;
    let mut state =
        tiny_glade_reconstruction::roof_replay::VisualReplayState::from_observed(raw_state);
    let mut changed_tick = 0;
    let outcome = tiny_glade_reconstruction::roof_replay::step_visual_replay_resolved(
        &mut roof,
        &mut state,
        &command,
        delta,
        &mut changed_tick,
        1,
    );
    Ok(format!(
        "{{\"baseline\":\"B\",\"resolved_only\":true,\"roof_hex\":\"{}\",\"elapsed_bits\":\"{:08x}\",\"finished\":{},\"changed_tick\":{},\"changed_fields\":{},\"feedback_count\":{}}}",
        roof.iter().map(|b| format!("{b:02x}")).collect::<String>(),
        state.elapsed().to_bits(),
        state.finished(),
        changed_tick,
        outcome.changed_fields,
        outcome.feedback.len()
    ))
}

fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("building-roof-create") if args.len() == 4 => match building_roof_command(&args) {
            Ok(result) => println!("{result}"),
            Err(error) => {
                eprintln!("building-roof-create: {error}");
                std::process::exit(1);
            }
        },
        Some("building-roof-update") if args.len() == 6 => match building_roof_command(&args) {
            Ok(result) => println!("{result}"),
            Err(error) => {
                eprintln!("building-roof-update: {error}");
                std::process::exit(1);
            }
        },
        Some("roof-history-step") if args.len() == 9 => match roof_history_step(&args) {
            Ok(result) => println!("{result}"),
            Err(error) => {
                eprintln!("roof-history-step: {error}");
                std::process::exit(1);
            }
        },
        Some("history-summary") if args.len() == 2 => {
            let result = std::fs::read(&args[1])
                .map_err(|e| e.to_string())
                .and_then(|bytes| tg_history::summarize_json(&bytes).map_err(|e| e.to_string()));
            match result {
                Ok(summary) => println!("{summary}"),
                Err(error) => {
                    eprintln!("history-summary: {error}");
                    std::process::exit(1);
                }
            }
        }
        Some("aspect") if args.len() == 3 => {
            let a = args[1].parse::<u32>().expect("first value must be u32");
            let b = args[2].parse::<u32>().expect("second value must be u32");
            let result = window_aspect::aspect_ratio_observed(a, b);
            println!(
                "{{\"value\":\"{result}\",\"f32_bits\":\"{:08x}\"}}",
                result.to_bits()
            );
        }
        Some("roof-state") if args.len() == 3 => {
            let a = args[1].parse::<f32>().expect("value at +0x30 must be f32");
            let b = args[2].parse::<f32>().expect("value at +0x34 must be f32");
            println!(
                "{{\"ty_byte\":{},\"is_gable\":{}}}",
                roof_state::ty_observed(a) as u8,
                roof_state::is_gable_observed(a, b)
            );
        }
        Some("identity") => println!("{{\"source_sha256\":\"{SOURCE_SHA256}\"}}"),
        Some("roof-section") if args.len() == 3 => {
            let roof = roof_bytes(&args[1]);
            let t = args[2]
                .parse::<f32>()
                .expect("section parameter must be f32");
            let mut out = [0xa5; 28];
            surface::get_roof_shape(&roof, t, &mut out);
            println!(
                "{{\"shape_bytes\":\"{}\"}}",
                out.iter().map(|b| format!("{b:02x}")).collect::<String>()
            );
        }
        Some("roof-ridge") if args.len() == 2 => {
            let roof = roof_bytes(&args[1]);
            let dims = ridge::ridge_dims_from_observed(&roof);
            match ridge::ridge_to_world_space_observed(dims, &roof) {
                Ok(ridge::RidgeGeometry::Point(p)) => {
                    println!("{{\"type\":\"point\",\"f32_bits\":{}}}", point_bits(p))
                }
                Ok(ridge::RidgeGeometry::Segment(p)) => println!(
                    "{{\"type\":\"segment\",\"f32_bits\":[{},{}]}}",
                    point_bits(p[0]),
                    point_bits(p[1])
                ),
                Err(e) => println!("{{\"error\":\"{e:?}\"}}"),
            }
        }
        Some("roof-profile") if args.len() == 5 => {
            let values: Vec<f32> = args[1..]
                .iter()
                .map(|s| s.parse().expect("profile arguments must be f32"))
                .collect();
            let points = surface::profile_curve_ws_points(
                &surface::profile_curve_normalized(values[0]),
                values[1],
                values[2],
                values[3],
            );
            let bits: Vec<_> = points
                .iter()
                .map(|p| format!("[\"{:08x}\",\"{:08x}\"]", p[0].to_bits(), p[1].to_bits()))
                .collect();
            println!("{{\"point_f32_bits\":[{}]}}", bits.join(","));
        }
        Some("roof-tiles") if (4..=5).contains(&args.len()) => {
            let roof = roof_bytes(&args[1]);
            let rect = if args[2] == "-" {
                None
            } else {
                let s = &args[2];
                assert_eq!(s.len(), 48, "rectangle must contain 24 bytes as hex");
                Some(std::array::from_fn::<_, 24, _>(|i| {
                    u8::from_str_radix(&s[2 * i..2 * i + 2], 16).unwrap()
                }))
            };
            let batch =
                roof_pipeline::assemble_roof_tiles_observed(&roof, rect.as_ref(), args[3] == "1")
                    .expect("invalid roof input");
            if let Some(path) = args.get(4) {
                let bytes: Vec<u8> = batch.records.iter().flat_map(|r| r.0).collect();
                std::fs::write(path, bytes).expect("unable to save packed tile records");
            }
            println!(
                "{{\"records\":{},\"record_size\":64,\"edges\":{},\"ridge_caps\":{},\"faces_and_fillers\":{}}}",
                batch.records.len(),
                batch.edges,
                batch.ridge_caps,
                batch.faces_and_fillers
            );
        }
        _ => {
            println!("Tiny Glade reconstruction: current verified algorithm runner");
            println!("Commands: identity | aspect <u32> <u32> | roof-state <f32> <f32>");
            println!(
                "roof-section <88-byte-hex> <t> | roof-ridge <88-byte-hex> | roof-profile <profile> <bottom> <top> <height>"
            );
            println!(
                "roof-tiles <88-byte-hex> <24-byte-rectangle-hex or -> <mode 0/1> [packed-record-output]"
            );
            println!("history-summary <history.json>");
            println!(
                "building-roof-create <B wall-state672hex> <create-event64hex> <initial-roof88hex>"
            );
            println!(
                "building-roof-update <B wall-state672hex> <roof88hex> <related48hex> <wall-id-hex> <entity-hex>"
            );
            println!(
                "roof-history-step <B history.json> <entry> <edit> <roof-hex> <duration> <elapsed> <dt> <undo 0/1>"
            );
        }
    }
}
