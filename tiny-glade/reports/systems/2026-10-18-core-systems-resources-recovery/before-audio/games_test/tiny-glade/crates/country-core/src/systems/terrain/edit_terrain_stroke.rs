//! B 0x14094e000 consumes point replacements before parameter edits. Missing
//! IDs are logged and skipped; old chapter's "missing stroke panic" was wrong.
//!
//! fresh 反编译（terrain-systems-0x14094e000）：
//! - 点集/编辑两循环、id 查找、错误串、`magnitude=delta*sqrt(dz²+(dy²+dx²))/0.001`、
//!   `apply_resolved` 逻辑、readback.pending LOCK INC 均吻合；
//! - **原版把 tag 1/2/3/4 观测写 `StairsPlacementBucket`（indexmap::Bucket<(DecoratorId,
//!   DecoratorId),AlignKind>）**（`grow_one<…>`），**无 PlayTerrainSound 载荷事件**——
//!   早期 Rust 的 PlayTerrainSound::Points/Falloff/Height/Removed 写入系**编造**
//!   （把 stairs bucket 记录误判为音频事件），已删。
use super::{
    delayed_terrain_changed_msg::ScheduledTerrainReadback, RecalculateHeightMapCmd,
    ui_edit_terrain::{StairsPlacementBucket, StairsPlacementEntry},
};
use crate::resources::{
    terrain_editing::{StrokeId, TerrainEditingData},
    time::GameTime,
};
use bevy_ecs::prelude::*;
use glam::Vec3;
use tg_terrain::edit::{apply_resolved, EditCommand, StrokeParameters};

#[derive(Event, Clone, Copy, Debug)]
pub struct EditTerrainStrokeCmd(pub EditCommand);

/// Native payload Vec@0, position@0x18, source ID@0x24; event ID follows at0x28.
#[derive(Event, Clone, Debug)]
pub struct SetTerrainStrokePtsCmd {
    pub points: Vec<[f32; 3]>,
    pub position: Vec3,
    pub id: StrokeId,
}

/// 原生 0x28B stairs bucket 记录（机械写偏移）：`+0x0c..+0x14` = position.x/y 位
/// （双 DecoratorId 键）、`+0x14` = position.z 位、`+0x18` = 值 a、`+0x1c` = 值 b、
/// `+0x20` = 幅度（tag 2/3）。
fn stairs_entry(
    tag: u32,
    position: Vec3,
    value_a: u32,
    value_b: u32,
    magnitude: f32,
) -> StairsPlacementEntry {
    StairsPlacementEntry {
        tag,
        decorator_a: position.x.to_bits(),
        decorator_b: position.y.to_bits(),
        sub: position.z.to_bits(),
        value_a,
        value_b,
        magnitude,
    }
}

pub fn edit_terrain_stroke(
    mut edits: EventReader<EditTerrainStrokeCmd>,
    mut point_sets: EventReader<SetTerrainStrokePtsCmd>,
    mut terrain: ResMut<TerrainEditingData>,
    mut stairs: ResMut<StairsPlacementBucket>,
    readback: Res<ScheduledTerrainReadback>,
    mut recalculate: EventWriter<RecalculateHeightMapCmd>,
    time: Res<GameTime>,
) {
    for command in point_sets.read() {
        let Some(stroke) = terrain.strokes.get_mut(&command.id) else {
            // fresh 第一循环错误串（@0x142ab3a20）："Was requested to set points of
            // terrain stroke {} that doesnt exist"（第二循环才是 "edit a terrain stroke"）。
            log::error!(
                "Was requested to set points of terrain stroke {} that doesnt exist",
                command.id.0
            );
            continue;
        };
        // Original indexing asserts before clearing the old point vector.
        let old = stroke.points[0];
        let new = command.points[0];
        let dx = old[0] - new[0];
        let dy = old[1] - new[1];
        let dz = old[2] - new[2];
        // RNE 0x14094e6be/6cd: y*y+x*x, then z*z + that sum.
        let magnitude =
            (time.delta * (dz * dz + (dy * dy + dx * dx)).sqrt()) / f32::from_bits(0x3a83126f);
        stroke.points.clone_from(&command.points);
        // tag=3（点集）stairs bucket：+0x18/+0x1c = stroke falloff/height 载荷。
        stairs.entries.push(stairs_entry(
            3,
            command.position,
            stroke.falloff.to_bits(),
            stroke.height_ws.to_bits(),
            magnitude,
        ));
        recalculate.write(RecalculateHeightMapCmd);
        readback.on_terrain_modified();
    }
    for event in edits.read() {
        let command = event.0;
        let Some(stroke) = terrain.strokes.get_mut(&command.id) else {
            log::error!(
                "Was requested to edit a terrain stroke {} that doesnt exist",
                command.id.0
            );
            continue;
        };
        // Reuse the already instruction-verified private game algorithm.
        let outcome = apply_resolved(
            StrokeParameters {
                falloff: stroke.falloff,
                height: stroke.height_ws,
            },
            command,
        );
        stroke.falloff = outcome.state.falloff;
        stroke.height_ws = outcome.state.height;
        let position = Vec3::from_array(command.position);
        for sample in outcome.audio {
            // 原版把 tag 1/2/4 观测写 StairsPlacementBucket（非音频事件）：
            // - tag=1（falloff）：+0x18 = 新 falloff、+0x1c = |delta|
            // - tag=2（height）：+0x18 = stroke falloff、+0x1c = 新 height、+0x20 = |delta|
            // - tag=4（removed）：+0x18 = stroke falloff、+0x1c = |delta|
            stairs.entries.push(match sample.tag {
                1 => stairs_entry(1, position, sample.falloff.to_bits(), sample.magnitude.to_bits(), 0.0),
                2 => stairs_entry(
                    2,
                    position,
                    stroke.falloff.to_bits(),
                    sample.height.expect("height stairs payload").to_bits(),
                    sample.magnitude,
                ),
                4 => stairs_entry(4, position, stroke.falloff.to_bits(), sample.magnitude.to_bits(), 0.0),
                _ => unreachable!("apply_resolved returned an unknown terrain stairs tag"),
            });
        }
        recalculate.write(RecalculateHeightMapCmd);
        readback.on_terrain_modified();
    }
}
