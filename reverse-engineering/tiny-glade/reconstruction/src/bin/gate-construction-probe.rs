pub use tiny_glade_reconstruction::{
    building_wall_collection, building_wall_construction_sources, building_wall_constructor,
    building_wall_production, building_wall_state, building_wall_structures, surface,
};
#[path = "../../building/gates.rs"]
mod gates;
use std::io::{self, BufRead};
fn decode(s: &str) -> Vec<f32> {
    (0..s.len())
        .step_by(8)
        .map(|i| {
            f32::from_bits(u32::from_le_bytes(
                (0..4)
                    .map(|j| u8::from_str_radix(&s[i + j * 2..i + j * 2 + 2], 16).unwrap())
                    .collect::<Vec<_>>()
                    .try_into()
                    .unwrap(),
            ))
        })
        .collect()
}
fn hex<I: IntoIterator<Item = u8>>(b: I) -> String {
    b.into_iter().map(|v| format!("{v:02x}")).collect()
}
fn floats<'a, I: IntoIterator<Item = &'a f32>>(fs: I) -> String {
    hex(fs.into_iter().flat_map(|f| f.to_le_bytes()))
}
fn main() {
    for l in io::stdin().lock().lines() {
        let l = l.unwrap();
        let w: Vec<_> = l.split_whitespace().collect();
        let p = decode(w[0]);
        let heights = decode(w[1]);
        let us = decode(w[2]);
        let pp = decode(w[3]);
        let pu = decode(w[4]);
        let scalar = decode(w[5])[0];
        let segment = building_wall_structures::MaterializedWallSegment {
            positions: p.chunks_exact(3).map(|v| v.try_into().unwrap()).collect(),
            heights,
            points_u: us,
            source_indices: [0, p.len() / 3],
            range: tiny_glade_reconstruction::building_wall_segmentation::WallLoopRange {
                start: 0,
                end: (p.len() / 3) as i32,
                modulus: (p.len() / 3) as i32,
            },
        };
        let prepared_positions: Vec<_> =
            pp.chunks_exact(3).map(|v| v.try_into().unwrap()).collect();
        let input = gates::GateConstructionInput {
            segment: &segment,
            prepared_positions: &prepared_positions,
            prepared_u: &pu,
            observed_public_288: scalar,
            observed_public_29a: w[6].parse().unwrap(),
            wall_id: building_wall_state::WallId(w[7].parse().unwrap()),
            ordinal: w[8].parse().unwrap(),
        };
        let terrain = building_wall_collection::TerrainHeightGrid512 {
            encoded: vec![0; 512 * 512].into(),
            minimum_y: 0.,
            maximum_y: 0.,
            sampling_fix: true,
        };
        let output = if w.get(9) == Some(&"system") {
            gates::construct_gate_cpu(input, &terrain)
        } else {
            gates::construct_wooden_gate_doors(input, &terrain)
        };
        match output {
            Ok(r) => {
                let nanis: Vec<_> = r
                    .nanis
                    .iter()
                    .map(|v| format!("\"{}\"", hex(v.canonical_bytes())))
                    .collect();
                let bricks: Vec<_> = r
                    .bricks
                    .iter()
                    .map(|v| format!("\"{}\"", hex(v.observed_bytes())))
                    .collect();
                let leaves:Vec<_>=r.leaves.iter().map(|g| format!(r#"{{"indices":"{}","attributes":{{"Vertex_Position":"{}","Vertex_Normal":"{}","Vertex_Color":"{}","Vertex_UV":"{}"}},"top_points":"{}","endpoint_directions":"{}","seed":{},"public_29a":{},"wall_id":{}}}"#,
      hex(g.mesh.indices.iter().flat_map(|v|v.to_le_bytes())),floats(g.mesh.positions.iter().flatten()),floats(g.mesh.normals.iter().flatten()),floats(g.mesh.colors.iter().flatten()),floats(g.mesh.uvs.iter().flatten()),floats(g.top_positions.iter().flatten()),floats(g.endpoint_directions.iter().flatten()),g.seed,g.observed_public_29a,g.wall_id.0)).collect();
                println!(
                    r#"{{"nanis":[{}],"bricks":[{}],"leaves":[{}]}}"#,
                    nanis.join(","),
                    bricks.join(","),
                    leaves.join(",")
                );
            }
            Err(e) => println!(r#"{{"error":"{e:?}"}}"#),
        }
    }
}
