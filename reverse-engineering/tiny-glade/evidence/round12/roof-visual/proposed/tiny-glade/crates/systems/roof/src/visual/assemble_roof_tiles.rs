//! B 141a36700: canonical event entity -> allocation clear -> original generator
//! -> SparseInstanceBufferWriter. Geometry algorithms remain in the existing core.
use bevy_ecs::prelude::*;
use country_core::{
    asset_libraries::ssbo_library::AssetSsboLibrary,
    resources::{
        roof_tile_ssbo::RoofTileSsbo,
        roofs::{Roof, RoofTiles, commands::RecalculateRoofCmd},
        walls::arches::ArchSegments,
    },
};

pub fn assemble_roof_tiles(
    mut recalc: EventReader<RecalculateRoofCmd>,
    mut tiles: Query<(&mut RoofTiles, &Roof)>,
    mut library: ResMut<AssetSsboLibrary>,
    arches: Res<ArchSegments>,
) {
    for event in recalc.read() {
        // Original looks up this storage before dereferencing the event entity.
        let buffer = library.get_mut::<RoofTileSsbo>().unwrap();
        let (mut tiles, roof) = tiles.get_mut(event.entity).unwrap();
        let tiles = &mut *tiles;
        let allocation = tiles
            .allocation
            .get_or_insert_with(|| buffer.new_allocation());
        let mut writer = buffer.writer(allocation);
        writer.clear();
        tiles.records.clear();
        if roof.is_flat() {
            continue;
        }
        let special_mode = arches.0.get(&roof.wall_id()).is_some_and(|v| !v.is_empty());
        let rectangle: &[u8; 24] = roof.observed()[0x0c..0x24].try_into().unwrap();
        let batch =
            country_core::resources::roofs::geometry::pipeline::assemble_roof_tiles_observed(
                roof.observed(),
                (roof.observed()[8] & 1 != 0).then_some(rectangle),
                special_mode,
            )
            .expect("original roof tile generator failed");
        for record in batch.records {
            let value = decode_record(&record.0);
            // Native closure logs once on each failure and returns to generation;
            // previously written records survive exhaustion. No batch rollback.
            if writer.push(&value).is_err() {
                warn_capacity(roof.observed()[8] & 1 != 0);
            } else {
                // Auxiliary CPU observation, not the owner used for rendering.
                tiles.records.push(record.0);
            }
        }
    }
}

fn decode_record(b: &[u8; 64]) -> RoofTileSsbo {
    let u = |o| u32::from_le_bytes(b[o..o + 4].try_into().unwrap());
    let f = |o| f32::from_bits(u(o));
    RoofTileSsbo {
        base_position: glam::Vec3::new(f(0), f(4), f(8)),
        current_height: f(12),
        clipping_plane: [u(16), u(20)],
        scale_t: [u(24), u(28)],
        rotation_quat: [u(32), u(36)],
        roof_id: u(40),
        velocity_rotation: u(44),
        animation_t: f(48),
        seed: u(52) as i32,
        is_circle: u(56) as i32,
        is_main_roof_body: u(60) as i32,
    }
}

fn warn_capacity(rectangular: bool) {
    use std::{collections::BTreeMap, sync::Mutex};
    // Original two monomorphized closures have independent macro expansion tables.
    static RECT: Mutex<BTreeMap<String, ()>> = Mutex::new(BTreeMap::new());
    static CIRCLE: Mutex<BTreeMap<String, ()>> = Mutex::new(BTreeMap::new());
    let message = "Out of roof tile instances";
    let table = if rectangular { &RECT } else { &CIRCLE };
    let fresh = table
        .lock()
        .expect("Mutex was poisonned")
        .insert(country_core::render::nani::warn_once_key(message), ())
        .is_none();
    if fresh {
        log::warn!(target: "system_roof::visual::assemble_roof_tiles", "{message}");
    }
}
