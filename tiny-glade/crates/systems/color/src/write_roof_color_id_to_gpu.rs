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

/// The original wall/gate/halftimber consumers perform the same write alongside
/// their CPU/audio/plaster work. Separate EventReaders preserve each event's
/// order while sharing the already registered authoritative CPU consumers.
pub fn write_wall_color_ids_to_gpu(
    device: Res<GraphicsDeviceResource>,
    library: Res<AssetSsboLibrary>,
    mut walls: EventReader<country_core::resources::color_id::SetWallColorIdCmd>,
    mut gates: EventReader<country_core::resources::color_id::SetGateColorIdCmd>,
    mut half: EventReader<country_core::resources::color_id::SetHalftimberColorIdCmd>,
) {
    use country_core::resources::color_id::{
        GateColorIdSsbo, HalftimberColorIdSsbo, WallColorIdSsbo,
    };
    let wall = library
        .get::<WallColorIdSsbo>()
        .expect("WallColorIdSsbo missing");
    let gate = library
        .get::<GateColorIdSsbo>()
        .expect("GateColorIdSsbo missing");
    let timber = library
        .get::<HalftimberColorIdSsbo>()
        .expect("HalftimberColorIdSsbo missing");
    for cmd in walls.read() {
        wall.gpu_write_with_offset(
            device.device(),
            &[WallColorIdSsbo(cmd.color)],
            cmd.wall_id().0 as usize * 4,
        )
        .expect("wall color GPU write failed");
    }
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
