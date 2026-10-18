// Recovered bodies to be merged into startup/load_prefabs.rs, not a runtime module.
use crate::render::mesh::{Mesh, VertexAttributeValues};
use glam::Vec3;
use std::collections::HashMap;

/// B 140a08680 / 140a08d30. Source requires RGB Float3; the predicate reads
/// only blue. Existing Mesh::retain_vertices performs original attribute
/// filtering and index remapping; do not implement a second remapper.
fn filter_mesh_by_vertex_color(mesh: &Mesh, canopy: bool) -> Mesh {
    let colors = match mesh.attributes.get("Vertex_Color") {
        Some(VertexAttributeValues::Float3(values)) => values,
        Some(_) => panic!("incorrect attribute type"),
        None => panic!("No Vertex_Color for mesh code processor"),
    };
    let mut keep = colors
        .iter()
        .map(|color| {
            i32::from(if canopy {
                color.z <= 0.0
            } else {
                color.z > 0.0
            })
        })
        .collect::<Vec<_>>();
    let mut output = mesh.clone();
    output.retain_vertices(&mut keep);
    output
}

fn float3_attribute<'a>(mesh: &'a Mesh, name: &str) -> &'a [Vec3] {
    match mesh.attributes.get(name).expect("no entry found for key") {
        VertexAttributeValues::Float3(values) => values,
        _ => panic!("incorrect attribute type"),
    }
}

/// B inlined 1412a2e51..39a8: prim_center's float BIT PATTERN is the quad key,
/// each quad accumulates original Vertex_Normal and all member indices. The
/// resulting normal is normalize_or_zero(sum); its center is mean(position).
/// This is not grouping by rounded coordinates or per-six-index triangles.
fn recalculate_canopy_quad_attribs(mesh: &mut Mesh) {
    struct Quad {
        members: Vec<u32>,
        normal: Vec3,
    }
    let centers = float3_attribute(mesh, "prim_center");
    let normals = float3_attribute(mesh, "Vertex_Normal");
    let positions = float3_attribute(mesh, "Vertex_Position");
    let mut quads = HashMap::<[u32; 3], Quad>::new();
    for (index, (center, normal)) in centers.iter().zip(normals).enumerate() {
        let key = center.to_array().map(f32::to_bits);
        let quad = quads.entry(key).or_insert_with(|| Quad {
            members: Vec::new(),
            normal: Vec3::ZERO,
        });
        quad.members.push(index as u32);
        quad.normal += *normal;
    }
    for quad in quads.values_mut() {
        quad.normal = quad.normal.normalize_or_zero();
    }
    let mut primitive_normals = vec![Vec3::ZERO; centers.len()];
    // Original assertion at 1412a368b precedes output expansion.
    assert_eq!(primitive_normals.len(), normals.len());
    for quad in quads.values() {
        for &index in &quad.members {
            primitive_normals[index as usize] = quad.normal;
        }
    }
    let mut primitive_centers = vec![Vec3::ZERO; centers.len()];
    for quad in quads.values() {
        let mut center = Vec3::ZERO;
        for &index in &quad.members {
            center += positions[index as usize];
        }
        center /= quad.members.len() as f32;
        for &index in &quad.members {
            primitive_centers[index as usize] = center;
        }
    }
    mesh.set_attribute("prim_normals", primitive_normals);
    mesh.set_attribute("prim_center", primitive_centers);
}

/// Original optional Code processor branches. Unknown names fail rather than
/// returning the raw input or an empty processed mesh.
fn process_prefab_mesh(source: &Mesh, processor: &str) -> Mesh {
    let canopy = match processor {
        "extract_canopy" => true,
        "extract_trunk" => false,
        _ => panic!("Unknown mesh code processor {processor:?}"),
    };
    let mut mesh = filter_mesh_by_vertex_color(source, canopy);
    if canopy && mesh.has_attribute("prim_center") {
        recalculate_canopy_quad_attribs(&mut mesh);
    }
    // Native both branches delete id, then add only missing appear_pos/age.
    let count = mesh
        .attributes
        .values()
        .next()
        .map_or(0, VertexAttributeValues::array_length);
    mesh.attributes.remove("id");
    if !mesh.has_attribute("appear_pos") {
        mesh.set_attribute("appear_pos", vec![Vec3::ZERO; count]);
    }
    if !mesh.has_attribute("age") {
        mesh.set_attribute("age", vec![1.0f32; count]);
    }
    mesh
}
