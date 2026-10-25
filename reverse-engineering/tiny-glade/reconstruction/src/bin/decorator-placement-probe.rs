use std::io::{self, BufRead};
use tiny_glade_reconstruction::{building_opening_rules, building_wall_state, surface};
#[path = "../../building/decorator_placement.rs"]
mod placement;
use placement::*;
fn f(s: &str) -> f32 {
    f32::from_bits(u32::from_str_radix(s, 16).unwrap())
}
fn floats(s: &str) -> Vec<f32> {
    s.split(',').map(f).collect()
}
fn emit(values: impl IntoIterator<Item = f32>) {
    println!(
        "{}",
        values
            .into_iter()
            .map(|x| format!("{:08x}", x.to_bits()))
            .collect::<Vec<_>>()
            .join(",")
    );
}
fn curve(p: &[&str], start: usize) -> surface::Curve2 {
    surface::Curve2 {
        length: f(p[start]),
        points: floats(p[start + 1])
            .chunks_exact(2)
            .map(|x| [x[0], x[1]])
            .collect(),
        points_u: floats(p[start + 2]),
    }
}
fn family(value: &str) -> building_opening_rules::OpeningFamily {
    match value {
        "cw" => building_opening_rules::OpeningFamily::CottageWindow,
        "gw" => building_opening_rules::OpeningFamily::GothicWindow,
        "cd" => building_opening_rules::OpeningFamily::CottageBalconyDoor,
        "gd" => building_opening_rules::OpeningFamily::GothicBalconyDoor,
        _ => panic!("unknown family"),
    }
}
fn encoded(values: impl IntoIterator<Item = f32>) -> String {
    values
        .into_iter()
        .map(|x| format!("{:08x}", x.to_bits()))
        .collect::<Vec<_>>()
        .join(",")
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        match p[0] {
            "asset" => println!(
                "{}",
                b_wall_collision_asset_name(family(p[1]), p[2].parse().unwrap()).unwrap()
            ),
            "bbox" => {
                let points: Vec<_> = floats(p[1])
                    .chunks_exact(3)
                    .map(|x| [x[0], x[1], x[2]])
                    .collect();
                match mesh_collision_bbx(&points) {
                    Ok(x) => emit(
                        x.center_xz
                            .into_iter()
                            .chain(x.extent_xz)
                            .chain(x.vertical_bounds),
                    ),
                    Err(e) => println!("error:{e:?}"),
                }
            }
            "from" => {
                let c = curve(&p, 1);
                let v = floats(p[4]);
                match wall_coord_from_world(&c, [v[0], v[1], v[2]]) {
                    Ok(x) => emit(x),
                    Err(e) => println!("error:{e:?}"),
                }
            }
            "to" => {
                let c = curve(&p, 1);
                let v = floats(p[4]);
                match wall_coord_to_world(&c, [v[0], v[1]]) {
                    Ok(x) => emit(x),
                    Err(e) => println!("error:{e:?}"),
                }
            }
            "key" => match wall_decorator_key(family(p[1]), p[2].parse().unwrap()) {
                Ok(x) => println!("{},{}", x.subtype, x.rank),
                Err(e) => println!("error:{e:?}"),
            },
            "place" => {
                let c = curve(&p, 4);
                let point = floats(p[7]);
                let vertices: Vec<_> = floats(p[8])
                    .chunks_exact(3)
                    .map(|x| [x[0], x[1], x[2]])
                    .collect();
                let mut shape = [0; 28];
                shape[..4].copy_from_slice(&p[3].parse::<u32>().unwrap().to_le_bytes());
                let shape = building_wall_state::WallShapeSnapshot::from_observed(shape);
                let request = WallDecoratorRequest {
                    family: family(p[1]),
                    rank: p[2].parse().unwrap(),
                    world_position: [point[0], point[1], point[2]],
                };
                let result = mesh_collision_bbx(&vertices)
                    .and_then(|collision| place_wall_decorator(request, &c, &shape, collision));
                match result {
                    Ok(x) => {
                        let holes =
                            building_opening_rules::generate_wall_opening(x.opening, &shape)
                                .unwrap();
                        let holes = holes
                            .into_iter()
                            .map(|h| {
                                format!(
                                    "{},{},{}",
                                    encoded(h.center),
                                    encoded(h.dimensions),
                                    h.kind
                                )
                            })
                            .collect::<Vec<_>>()
                            .join(";");
                        println!(
                            "{},{}|{}|{}|{}",
                            x.key.subtype,
                            x.key.rank,
                            encoded(x.opening.center),
                            encoded(x.projected_world_position),
                            holes
                        );
                    }
                    Err(e) => println!("error:{e:?}"),
                }
            }
            _ => panic!("unknown operation"),
        }
    }
}
