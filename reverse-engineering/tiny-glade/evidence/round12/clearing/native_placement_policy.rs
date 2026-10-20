use glam::{Affine3A, Quat, Vec3};

/// Original init_clearing closure0 keeps exterior placements, including the
/// exact +/-65 boundary and unordered coordinates. Preserve its two compares.
pub fn is_interactable_forest_placement(position: Vec3) -> bool {
    !(position.x.abs() < 65.0 && position.z.abs() < 65.0)
}

/// Original closure1: full quaternion/affine transform through the original
/// dependency, retaining filter order, stored angle units, and all four words.
pub fn particle_emitter_for_vertex(
    position: Vec3,
    scale: f32,
    rotation: f32,
    vertex: Vec3,
) -> Option<[u32; 4]> {
    let radius =
        (position.z * position.z + (position.x * position.x + position.y * position.y)).sqrt();
    if !(radius < 45.0) || !(scale > 0.7) {
        return None;
    }
    let affine = Affine3A::from_scale_rotation_translation(
        Vec3::splat(scale),
        Quat::from_rotation_y(rotation),
        position,
    );
    let point = affine.transform_point3(vertex);
    Some([point.x.to_bits(), point.y.to_bits(), point.z.to_bits(), 0])
}
