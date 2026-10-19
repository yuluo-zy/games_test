//! B 1421a6d40, 839 bytes. Original descriptor mutation on the real roof entity.
use bevy_ecs::prelude::*;
use country_core::{
    components::drawable::NaniInstanceDesc,
    render::nani::{gpu_world::GpuWorldSubsetKind, systems::NaniMeshSelector},
    resources::roofs::{Roof, commands::RecalculateRoofCmd},
};

pub fn generate_roof_stone_floor_and_roof_bottom(
    mut recalc: EventReader<RecalculateRoofCmd>,
    roofs: Query<(Entity, &Roof)>,
    mut commands: Commands,
) {
    for event in recalc.read() {
        let (entity, roof) = roofs.get(event.entity).unwrap();
        let part = if roof.observed()[8] & 1 != 0 {
            0
        } else {
            let radius = f32::from_le_bytes(roof.observed()[0x14..0x18].try_into().unwrap());
            let value = (radius * 3.0).log2();
            // MAXSS(1,value), then MINSS(5,result): second operand wins on NaN.
            let low = if 1.0 > value { 1.0 } else { value };
            let high = if 5.0 < low { 5.0 } else { low };
            high as u32
        };
        commands.entity(entity).insert(NaniInstanceDesc {
            mesh: NaniMeshSelector::AtlasPart(part),
            subset_kind: GpuWorldSubsetKind::RoofFloor,
        });
    }
}
