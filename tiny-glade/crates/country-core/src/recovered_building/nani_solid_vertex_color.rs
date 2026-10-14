//! SolidVertexColor's private upload schema, recovered from the original B
//! writer call. Parallel implementation: build/native/World verification deferred.
//! Quaternion/matrix operations belong to the pinned glam dependency.
#[derive(Clone, Debug, PartialEq)]
pub struct SolidVertexColorCpuProjection {
    pub record: [u8; 64],
    pub bounds_center: [f32; 3],
    pub bounds_radius: f32,
}

/// Both flag values must be supplied by their actual component/query owner.
/// Gate bundle metadata is a different field and must not be used as flags.
pub fn project_solid_vertex_color(
    transform: &[u8; 48],
    observed_flag_30: bool,
    observed_flag_34: bool,
) -> SolidVertexColorCpuProjection {
    let f = |offset| f32::from_le_bytes(transform[offset..offset + 4].try_into().unwrap());
    let rotation = glam::Mat3::from_quat(glam::Quat::from_xyzw(f(0), f(4), f(8), f(12)));
    let scale = glam::Vec3::new(f(28), f(32), f(36));
    let scaled = glam::Mat3::from_cols(
        rotation.x_axis * scale.x,
        rotation.y_axis * scale.y,
        rotation.z_axis * scale.z,
    );
    let center = [f(16), f(20), f(24)];
    let mut record = [0; 64];
    for (index, value) in scaled.to_cols_array().into_iter().chain(center).enumerate() {
        record[index * 4..index * 4 + 4].copy_from_slice(&value.to_le_bytes());
    }
    record[48..52].copy_from_slice(&u32::from(observed_flag_30).to_le_bytes());
    record[52..56].copy_from_slice(&u32::from(observed_flag_34).to_le_bytes());
    SolidVertexColorCpuProjection {
        record,
        bounds_center: center,
        bounds_radius: scale.z.abs().max(scale.y.abs()).max(scale.x.abs()),
    }
}
