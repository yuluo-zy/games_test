use std::io::{self, BufRead};
#[path = "../../building/wall_authoring.rs"]
mod authoring;
#[path = "../../building/roof_invalid_notifications.rs"]
mod invalid_roofs;
use tiny_glade_reconstruction::{
    building_history_transactions, building_roof_creation, building_wall_collection, building_wall_freeform_cache, building_wall_state, roof_parameters,
};
use authoring::{AuthoringShape, SourceMutation};
use building_wall_freeform_cache::{PreviewCircleObserved, PreviewRectangleObserved};
use building_wall_state::WallId;

fn bytes<const N: usize>(v: &str) -> [u8; N] {
    std::array::from_fn(|i| u8::from_str_radix(&v[i * 2..i * 2 + 2], 16).unwrap())
}
fn hex(v: &[u8]) -> String { v.iter().map(|b| format!("{b:02x}")).collect() }
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "invalid" => {
                let roofs: Vec<[u8; 88]> = if p[3] == "-" { vec![] } else { p[3].split(',').map(bytes).collect() };
                let rows: Vec<_> = roofs.iter().map(|roof| invalid_roofs::InvalidRoofRow { roof, marked_for_deletion: p[1] == "1" }).collect();
                let ids: Vec<_> = if p[4] == "-" { vec![] } else { p[4].split(',').map(|id| WallId(id.parse().unwrap())).collect() };
                let changes = invalid_roofs::process_invalid_roof_notifications(&rows, &ids, p[2] == "1");
                println!("{}", changes.iter().map(|c| format!("{}:{}:{}", c.wall_id.0, c.observed_delete_flag, c.history_mutation.as_ref().map_or("-".into(), |h| hex(h.payload())))).collect::<Vec<_>>().join("|"));
            }
            "edit" => {
                let metadata: [u8; 112] = bytes(p[3]);
                let read = |at| f32::from_le_bytes(metadata[at..at + 4].try_into().unwrap());
                let shape = authoring::AuthoringShape::Rectangle(PreviewRectangleObserved { axes: [1.0, 0.0], center: [0.0, 0.0], dimensions: [2.0, 3.0] });
                let mut counter = 0;
                let mut source = authoring::create_shape_source(&mut counter, authoring::CreateShapeRequest { shape, previous_wall_hint: None, height: read(0x54), elevation: read(0x58), selected_style: metadata[0x68] }).unwrap().source;
                source.observed_source_flag_64 = metadata[0x64]; source.observed_path_scalar = read(0x5c); source.special = metadata[0x65] != 0; source.clutter_version = metadata[0x66];
                let event: [u8; 64] = bytes(p[2]);
                let outcome = authoring::edit_rectangle_source((p[1] == "1").then_some(&mut source), &event, f32::from_bits(u32::from_str_radix(p[4], 16).unwrap())).unwrap();
                let points: Vec<u8> = source.curve.points.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
                let us: Vec<u8> = source.curve.points_u.iter().flat_map(|f| f.to_le_bytes()).collect();
                let anchor: Vec<u8> = outcome.audio_anchor.iter().flat_map(|f| f.to_le_bytes()).collect();
                println!("{} {} {} {} {:08x} {} {} {:08x} {} {:08x} {:08x} {:08x} {} {} {} {} {}", outcome.wall_id.0, u8::from(outcome.updated_source_notifications.is_some()), hex(source.shape.bytes()), hex(&points), source.curve.length.to_bits(), hex(&us), hex(&anchor), outcome.motion_audio_amplitude.to_bits(), u8::from(outcome.motion_audio_flag), source.extra.to_bits(), source.terrain_floor.to_bits(), source.observed_path_scalar.to_bits(), source.style, u8::from(source.special), source.clutter_version, outcome.material_audio_kind, source.observed_source_flag_64);
            }
            "building" => {
                let mut counter = p[1].parse().unwrap();
                let raw: [u8; 28] = bytes(p[2]);
                let read = |at| f32::from_le_bytes(raw[at..at + 4].try_into().unwrap());
                let shape = if raw[0] == 0 { AuthoringShape::Circle(PreviewCircleObserved { center: [read(4), read(8)], radius: read(12), angle: read(16) }) }
                    else { AuthoringShape::Rectangle(PreviewRectangleObserved { axes: [read(4), read(8)], center: [read(12), read(16)], dimensions: [read(20), read(24)] }) };
                let settings = authoring::CreationSettingsObserved { wall_color_id: p[4].parse().unwrap(), roof_color_id: p[5].parse().unwrap(), timber_color_id: p[6].parse().unwrap(), wall_style: p[7].parse().unwrap(), rim_style: p[8].parse().unwrap() };
                let previous = if p[9] == "none" { None } else {
                    let b: [u8; 32] = bytes(p[9]);
                    Some(authoring::PreviousBuildingObserved { public_point_count: u64::from_le_bytes(b[..8].try_into().unwrap()), public_raw_length: f32::from_le_bytes(b[8..12].try_into().unwrap()), public_shape_tag: u32::from_le_bytes(b[12..16].try_into().unwrap()), public_style: b[16], wall_color_id: u32::from_le_bytes(b[20..24].try_into().unwrap()), timber_color_id: u32::from_le_bytes(b[24..28].try_into().unwrap()) })
                };
                let roof = if p[10] == "none" { None } else { Some(bytes::<88>(p[10])) };
                match authoring::create_building_from_placement(&mut counter, shape, &bytes(p[3]), settings, previous, roof.as_ref()) {
                    Ok(b) => {
                        let s = b.wall.source;
                        let points: Vec<u8> = s.curve.points.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
                        let us: Vec<u8> = s.curve.points_u.iter().flat_map(|f| f.to_le_bytes()).collect();
                        println!("{} {} {} {:08x} {:08x} {:08x} {} {} {:08x} {} {} {} {} {} {}",
                            b.wall.wall_id.0, counter, s.style, s.extra.to_bits(), s.terrain_floor.to_bits(), s.observed_path_scalar.to_bits(), hex(s.shape.bytes()), hex(&points), s.curve.length.to_bits(), hex(&us), b.appearance.wall_color_id, b.appearance.roof_color_id, b.appearance.timber_color_id, hex(b.roof_request.record()), s.observed_source_flag_64);
                    }
                    Err(e) => println!("error {e:?}"),
                }
            }
            "create" => {
                let mut counter = p[1].parse().unwrap();
                let style = p[2].parse().unwrap();
                let shape: [u8; 28] = bytes(p[3]);
                let read = |at| f32::from_le_bytes(shape[at..at + 4].try_into().unwrap());
                let shape = match u32::from_le_bytes(shape[..4].try_into().unwrap()) {
                    0 => AuthoringShape::Circle(PreviewCircleObserved { center: [read(4), read(8)], radius: read(12), angle: read(16) }),
                    1 => AuthoringShape::Rectangle(PreviewRectangleObserved { axes: [read(4), read(8)], center: [read(12), read(16)], dimensions: [read(20), read(24)] }),
                    _ => panic!("Only original circle/rectangle authoring is in scope"),
                };
                let request = authoring::request_from_shape_placement(shape, &bytes(p[4]), style);
                match authoring::create_shape_source(&mut counter, request) {
                    Ok(v) => {
                        let s = v.source;
                        let points: Vec<u8> = s.curve.points.iter().flatten().flat_map(|f| f.to_le_bytes()).collect();
                        let us: Vec<u8> = s.curve.points_u.iter().flat_map(|f| f.to_le_bytes()).collect();
                        println!("{} {} {} {} {:08x} {:08x} {:08x} {} {} {} {} {} {:08x} {}",
                            v.wall_id.0, counter, v.new_source_notification.0, v.updated_source_notification.0,
                            s.extra.to_bits(), s.terrain_floor.to_bits(), s.observed_path_scalar.to_bits(),
                            s.style, u8::from(s.special), s.clutter_version, hex(s.shape.bytes()), hex(&points), s.curve.length.to_bits(), s.observed_source_flag_64);
                        println!("u {}", hex(&us));
                    }
                    Err(e) => println!("error {e:?}"),
                }
            }
            "event" => {
                let raw: [u8; 28] = bytes(p[1]);
                let read = |at| f32::from_le_bytes(raw[at..at + 4].try_into().unwrap());
                let shape = match u32::from_le_bytes(raw[..4].try_into().unwrap()) {
                    0 => AuthoringShape::Circle(PreviewCircleObserved { center: [read(4), read(8)], radius: read(12), angle: read(16) }),
                    1 => AuthoringShape::Rectangle(PreviewRectangleObserved { axes: [read(4), read(8)], center: [read(12), read(16)], dimensions: [read(20), read(24)] }),
                    _ => panic!("create event shape"),
                };
                let request = authoring::request_from_shape_placement(shape, &bytes(p[2]), 0);
                println!("{}", hex(&authoring::active_create_event_bytes(request)));
            }
            "preset" => {
                let raw: [u8; 28] = bytes(p[1]);
                let read = |at| f32::from_le_bytes(raw[at..at + 4].try_into().unwrap());
                let shape = match u32::from_le_bytes(raw[..4].try_into().unwrap()) {
                    0 => AuthoringShape::Circle(PreviewCircleObserved { center: [read(4), read(8)], radius: read(12), angle: read(16) }),
                    1 => AuthoringShape::Rectangle(PreviewRectangleObserved { axes: [read(4), read(8)], center: [read(12), read(16)], dimensions: [read(20), read(24)] }),
                    _ => panic!("preset shape"),
                };
                let roof = if p[3] == "none" { None } else { Some(bytes::<88>(p[3])) };
                println!("{}", hex(authoring::creation_roof_parameters(shape, p[2].parse().unwrap(), roof.as_ref()).bytes()));
            }
            "mutate" => {
                let mut source = bytes(p[4]);
                let mutation = match p[1] {
                    "elevation" => SourceMutation::Elevation(u32::from_str_radix(p[5], 16).unwrap()),
                    "enclosure" => SourceMutation::Enclosure(bytes(p[5])),
                    "style" => SourceMutation::Style(p[5].parse().unwrap()),
                    _ => panic!("source mutation"),
                };
                let result = authoring::mutate_observed_source((p[2] == "1").then_some(&mut source), WallId(p[3].parse().unwrap()), mutation);
                println!("{} {}", hex(&source), result.map_or("-".into(), |id| id.0.to_string()));
            }
            "remove" => {
                let present = p[1] == "1";
                let wall = WallId(p[2].parse().unwrap());
                let source: [u8; 112] = bytes(p[3]);
                let mut map = std::collections::HashMap::new();
                if present { map.insert(wall, source); }
                else if wall.0 & 1 != 0 { map.insert(WallId(wall.0 ^ 1), source); }
                let mut removed = std::collections::HashSet::new();
                match authoring::remove_source(&mut map, &mut removed, wall) {
                    Ok(source) => println!("{} {} {}", hex(&source), map.len(), u8::from(removed.contains(&wall))),
                    Err(e) => println!("error {e:?} {} {}", map.len(), removed.len()),
                }
            }
            "inactive" => {
                let mut preview: [u8; 40] = bytes(p[4]);
                let changed = authoring::clear_preview_for_inactive_mode(p[1].parse().unwrap(), p[2].parse().unwrap(), p[3].parse().unwrap(), &mut preview);
                println!("{} {}", u8::from(changed), hex(&preview));
            }
            "appearance" => {
                let settings = authoring::CreationSettingsObserved {
                    wall_color_id: p[1].parse().unwrap(), roof_color_id: p[2].parse().unwrap(),
                    timber_color_id: p[3].parse().unwrap(), wall_style: p[4].parse().unwrap(), rim_style: 0,
                };
                let previous = if p[5] == "none" { None } else {
                    let b: [u8; 32] = bytes(p[5]);
                    Some(authoring::PreviousBuildingObserved {
                        public_point_count: u64::from_le_bytes(b[..8].try_into().unwrap()),
                        public_raw_length: f32::from_le_bytes(b[8..12].try_into().unwrap()),
                        public_shape_tag: u32::from_le_bytes(b[12..16].try_into().unwrap()),
                        public_style: b[16], wall_color_id: u32::from_le_bytes(b[20..24].try_into().unwrap()),
                        timber_color_id: u32::from_le_bytes(b[24..28].try_into().unwrap()),
                    })
                };
                let roof = if p[6] == "none" { None } else { Some(bytes::<88>(p[6])) };
                match authoring::select_creation_appearance(settings, previous, roof.as_ref()) {
                    Ok(selected) => println!("{} {} {} {} {}", selected.wall_color_id, selected.roof_color_id, selected.timber_color_id, selected.wall_style, hex(&selected.roof_appearance)),
                    Err(e) => println!("error {e:?}"),
                }
            }
            _ => panic!("authoring probe mode"),
        }
    }
}
