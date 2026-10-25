//! B `systems/wall/eraser/ui_bulldozer.rs`: bulldozer UI entry.
//!
//! Evidence: `chapters/systems/wall/eraser/ui_bulldozer.rs.md` (O,
//! 0x1409de190 / condition 0x1409de170). The condition is
//! `AppMode::Eraser(EraserLayer::Bulldoze)` (native tag 9, sub-discriminant
//! 3). **归属核对（fresh 2026-10-17）**：0x1409de190 本体是 **UI 生产者**——
//! `UiState::add` + `Rapier::raycast_simple`（0x1408ce180，collision 域）→
//! 命中类型跳表（case 0x11 demo / 0-2,6-9 墙 / 4 / 5 / 13 / 14...）→ 逐墙记录
//! 距离 → "delete_tree"/"cursor_hammer" signifier 写 + UiState::add<String>
//! → 末 jumptable 动作分派。Rust 此前把 **事件→缓冲桥**（AnimateWallBulldozeCmd
//! 读 → AnimateWallBulldozeBuffer 入队）误挂到本 VA——该缓冲入队是 native 跳表
//! 目标 handler（独立 VA，不在 0x1409de190 体内），已拆为
//! [`enqueue_bulldoze_command`]。
//!
//! 登记：Rapier::raycast_simple 为 collision 域接口（以 [`BulldozerRayPick`]
//! 代理），命中类型跳表/距离筛选/DebugViz 为宿主接线。

use bevy_ecs::prelude::*;

use super::animate_wall_bulldoze::AnimateWallBulldozeBuffer;
use super::AnimateWallBulldozeCmd;
use crate::systems::wall::eraser::bulldoze::WallBulldozeCommand;
use crate::resources::walls::{InnerWalls, WallId};
use crate::systems::mode_manager::{AppMode, EraserLayer};
use crate::systems::ui::signifiers::signifier_create::{UiAppendSignifier, UiSignifierStream};

/// B 0x1409de170: `(*app).tag == 9 && (*app).byte@+4 == 3` = bulldozer layer.
pub fn is_bulldozer_layer(mode: &AppMode) -> bool {
    matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze))
}

/// B 0x1409de170 as a run condition.
pub fn wall_eraser_bulldoze_condition(mode: Res<AppMode>) -> bool {
    is_bulldozer_layer(&mode)
}

/// Ray-pick 接口代理（原生 `Rapier::raycast_simple` 0x1408ce180，collision 域
/// 写入）。本域以 `hit`（命中的墙）+ `position`（命中点）承载 ray-pick 结果；
/// Rapier 射线为 collision 域接口，登记不编造。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct BulldozerRayPick {
    /// 命中的墙（collision 域 ray-pick 命中墙时写）。
    pub hit: Option<WallId>,
    /// 命中点世界坐标。
    pub position: [f32; 3],
}

/// 原生 "delete_tree" signifier 记录（`PTR_s_delete_tree_` 142ac3c18 格式化 +
/// AppendOnlyVec::push）；name 槽 = 格式化名字句柄（语义 U）。
pub fn bulldoze_signifier(wall: WallId, position: [f32; 3]) -> UiAppendSignifier {
    UiAppendSignifier {
        tag: 1,
        kind: 1,
        position: glam::Vec3::new(position[0], position[1], position[2]),
        scale: 1.0,
        name: wall.0 as u64,
    }
}

/// B 0x1409de190 (UI 生产者)：ray-pick 命中 → "delete_tree" signifier +
/// AnimateWallBulldozeCmd 动作分派（native 末 jumptable 目标 handler =
/// [`enqueue_bulldoze_command`]）。
pub fn wall_eraser_bulldoze(
    ray: Res<BulldozerRayPick>,
    mut commands: EventWriter<AnimateWallBulldozeCmd>,
    mut stream: ResMut<UiSignifierStream>,
) {
    let Some(wall) = ray.hit else {
        return;
    };
    stream.write(bulldoze_signifier(wall, ray.position));
    // 动作分派：low/high 擦除 u 范围由原生命中类型跳表目标 handler 决定（待接线，
    // 全墙擦除 low=0/high=1 登记）。
    commands.write(AnimateWallBulldozeCmd {
        wall,
        low: 0.0,
        high: 1.0,
        anchor: ray.position,
        flag: 0,
    });
}

/// native 跳表目标 handler（缓冲入队，非 0x1409de190）：enqueue the bulldoze
/// command, skipping already-animating walls (the original scans
/// AnimateWallBulldozeBuffer by WallId).
pub fn enqueue_bulldoze_command(
    mut commands: EventReader<AnimateWallBulldozeCmd>,
    mut buffer: ResMut<AnimateWallBulldozeBuffer>,
    inner: Res<InnerWalls>,
) {
    for command in commands.read() {
        if buffer.contains(command.wall) {
            continue;
        }
        let Some(state) = inner.get(command.wall).cloned() else {
            continue;
        };
        buffer.push(WallBulldozeCommand {
            original: state.snapshot().clone(),
            wall: crate::resources::walls::publication_fields::WallId(command.wall.0),
            low: command.low,
            high: command.high,
            anchor: command.anchor,
            phase: 0.0,
            finished: false,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bulldoze_layer_condition() {
        assert!(is_bulldozer_layer(&AppMode::Eraser(EraserLayer::Bulldoze)));
        assert!(!is_bulldozer_layer(&AppMode::Eraser(EraserLayer::Wall)));
        assert!(!is_bulldozer_layer(&AppMode::Wall(
            crate::systems::mode_manager::WallShape::Freeform
        )));
    }

    #[test]
    fn producer_writes_signifier_and_dispatches_command_on_hit() {
        let record = bulldoze_signifier(WallId(7), [1.0, 2.0, 3.0]);
        assert_eq!((record.tag, record.kind, record.name), (1, 1, 7));
        assert_eq!(record.position, glam::Vec3::new(1.0, 2.0, 3.0));

        let mut world = World::new();
        world.insert_resource(BulldozerRayPick {
            hit: Some(WallId(7)),
            position: [1.0, 2.0, 3.0],
        });
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<AnimateWallBulldozeCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(wall_eraser_bulldoze);
        schedule.run(&mut world);
        let cmds = world
            .resource_mut::<Events<AnimateWallBulldozeCmd>>()
            .drain()
            .collect::<Vec<_>>();
        assert_eq!(cmds.len(), 1);
        assert_eq!(cmds[0].wall, WallId(7));
        // 无命中 → 无命令分派。
        let mut world = World::new();
        world.insert_resource(BulldozerRayPick::default());
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<Events<AnimateWallBulldozeCmd>>();
        let mut schedule = Schedule::default();
        schedule.add_systems(wall_eraser_bulldoze);
        schedule.run(&mut world);
        assert_eq!(
            world
                .resource_mut::<Events<AnimateWallBulldozeCmd>>()
                .drain()
                .count(),
            0
        );
    }
}

/// B 0x1409e35a0: hammer cursor icon system (UI layer, 4.4 KB, chapter I).
/// The cursor icon swap is render/UI host 待接线; the library exposes the
/// bulldozer-active predicate it is keyed on.
pub fn hammer_cursor_icon() -> bool {
    is_bulldozer_layer(&crate::systems::mode_manager::AppMode::Eraser(
        EraserLayer::Bulldoze,
    ))
}
