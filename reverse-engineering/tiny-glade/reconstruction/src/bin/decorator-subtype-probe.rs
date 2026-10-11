use std::io::{self, BufRead};
use tiny_glade_reconstruction::{
    building_decorator_placement, building_opening_rules, building_wall_state, roof_state, surface,
};
#[path = "../../building/decorator_subtype.rs"]
mod subtype;
fn floats(s: &str) -> Vec<f32> {
    s.split(',')
        .map(|s| f32::from_bits(u32::from_str_radix(s, 16).unwrap()))
        .collect()
}
fn main() {
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let p: Vec<_> = line.split_whitespace().collect();
        if p[0] == "asset" {
            println!(
                "{}",
                subtype::b_wall_subtype_collision_asset_name(
                    p[1].parse().unwrap(),
                    p[2].parse().unwrap()
                )
                .unwrap()
            );
            continue;
        }
        let kind = if p[0] == "cw" {
            subtype::WallWindowKind::Cottage
        } else {
            subtype::WallWindowKind::Gothic
        };
        let c = floats(p[3]);
        let raw = floats(p[5]);
        let curve = surface::Curve2 {
            length: floats(p[4])[0],
            points: raw.chunks_exact(2).map(|v| [v[0], v[1]]).collect(),
            points_u: floats(p[6]),
        };
        let outline = floats(p[7]);
        let outline: Vec<_> = outline
            .chunks_exact(3)
            .map(|v| [v[0], v[1], v[2]])
            .collect();
        let heights = floats(p[8]);
        let mut shape = [0; 28];
        shape[..4].copy_from_slice(&p[9].parse::<u32>().unwrap().to_le_bytes());
        let shape = building_wall_state::WallShapeSnapshot::from_observed(shape);
        let roof = if p[10] == "none" {
            None
        } else {
            let bytes: Vec<_> = (0..p[10].len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&p[10][i..i + 2], 16).unwrap())
                .collect();
            Some(<[u8; 88]>::try_from(bytes).unwrap())
        };
        let bounds = floats(p[11]);
        let bbx = building_decorator_placement::CollisionBoundingBox {
            center_xz: [0.; 2],
            extent_xz: [0.; 2],
            vertical_bounds: [bounds[0], bounds[1]],
        };
        let ctx = subtype::WallDecoratorSubtypeContext {
            source_curve: &curve,
            mapped_outline: &outline,
            arch_heights: &heights,
            wall_shape: &shape,
            roof: roof.as_ref(),
        };
        match subtype::derive_wall_subtype(kind, [c[0], c[1]], p[1] == "1", p[2] == "1", ctx, bbx) {
            Ok(v) => {
                if p.len() > 12 {
                    let vertices = floats(p[12]);
                    let vertices: Vec<_> = vertices
                        .chunks_exact(3)
                        .map(|p| [p[0], p[1], p[2]])
                        .collect();
                    let collision =
                        building_decorator_placement::mesh_collision_bbx(&vertices).unwrap();
                    let input = building_opening_rules::DecoratorOpeningInput {
                        family: subtype::wall_opening_family(v).unwrap(),
                        size_variant: p[13].parse().unwrap(),
                        center: [c[0], c[1]],
                        collision: collision.metrics(),
                    };
                    let holes =
                        building_opening_rules::generate_wall_opening(input, &shape).unwrap();
                    let encoded =
                        |v: [f32; 2]| format!("{:08x},{:08x}", v[0].to_bits(), v[1].to_bits());
                    println!(
                        "{v}|{}",
                        holes
                            .into_iter()
                            .map(|h| format!(
                                "{},{},{}",
                                encoded(h.center),
                                encoded(h.dimensions),
                                h.kind
                            ))
                            .collect::<Vec<_>>()
                            .join(";")
                    );
                } else {
                    println!("{v}");
                }
            }
            Err(v) => println!("error:{v:?}"),
        }
    }
}
