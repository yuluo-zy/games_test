//! B142298520: creation source first, then color-command source; four-byte
//! writes at wall_index*4 through the existing typed buffer/device owner.
use crate::SetRoofColorIdCmd;
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    render::device::GraphicsDeviceResource,
    resources::{color_id_roof::RoofColorIdSsbo, roofs::commands::CreateRoofCmd},
};

pub fn write_roof_color_id_to_gpu(
    device: Res<GraphicsDeviceResource>,
    library: Res<AssetSsboLibrary>,
    mut roof_cmds: EventReader<SetRoofColorIdCmd>,
    mut creates: EventReader<CreateRoofCmd>,
) {
    let buffer = library
        .get::<RoofColorIdSsbo>()
        .expect("RoofColorIdSsbo missing");
    for cmd in creates.read() {
        let color = u32::from_le_bytes(cmd.0.record()[16..20].try_into().unwrap());
        buffer
            .gpu_write_with_offset(
                device.device(),
                &[RoofColorIdSsbo(color)],
                cmd.0.wall_id().0 as usize * 4,
            )
            .expect("roof color GPU write failed");
    }
    for cmd in roof_cmds.read() {
        buffer
            .gpu_write_with_offset(
                device.device(),
                &[RoofColorIdSsbo(cmd.color)],
                cmd.id as usize * 4,
            )
            .expect("roof color GPU write failed");
    }
}

/// 尚未迁回原setter的门／半木上传适配；墙表已由真实setter原地整表上传。
/// 不再重复消费墙颜色事件，也不将其延迟到set40之后。
pub fn write_gate_and_halftimber_color_ids_to_gpu(
    device: Res<GraphicsDeviceResource>,
    library: Res<AssetSsboLibrary>,
    mut gates: EventReader<country_core::resources::color_id::SetGateColorIdCmd>,
    mut half: EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,
) {
    use country_core::resources::color_id::{
        GateColorIdSsbo, HalftimberColorIdSsbo,
    };
    let gate = library
        .get::<GateColorIdSsbo>()
        .expect("GateColorIdSsbo missing");
    let timber = library
        .get::<HalftimberColorIdSsbo>()
        .expect("HalftimberColorIdSsbo missing");
    for cmd in gates.read() {
        gate.gpu_write_with_offset(
            device.device(),
            &[GateColorIdSsbo(cmd.color)],
            cmd.wall_id().0 as usize * 4,
        )
        .expect("gate color GPU write failed");
    }
    for cmd in half.read() {
        timber
            .gpu_write_with_offset(
                device.device(),
                &[HalftimberColorIdSsbo(cmd.color)],
                cmd.wall_id().0 as usize * 4,
            )
            .expect("halftimber color GPU write failed");
    }
}
