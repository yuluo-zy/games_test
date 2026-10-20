//! `country_core::systems::wall::adjust_wall_height::adjust_wall_height_impl::adjust_wall_height_impl`
//! （B 0x140ac1f20，2235 B；调用包装 `InfallibleSystemWrapper::run_unsafe` 0x1402516dc 之外的
//! 0x14015c162）。
//!
//! 原版 SystemParam 面（反编译 8 个形参，顺序固定）：
//!
//! | # | 原版形参 | 本实现 |
//! |---|---|---|
//! | 1 | `EventReader<AdjustWallHeightCmd>`（双缓冲游标，每事件 0x30 B） | 同 |
//! | 2 | `ResMut<InnerWalls>`（map 项 0x78 B，高度 `+0x5c`，曲线类别 `+0x40`） | 同 |
//! | 3 | `EventWriter<RequestWallMaterialAudio>`（`Events` 项 0x20 B：id/wall、pos、kind、flag） | 同 |
//! | 4 | `EventWriter<PlayBuildingAudio>`（项 0x28 B：id/wall、pos、f32、kind） | 同 |
//! | 5 | `Res<terrain height>`（`sample_at_world_xz`） | `Res<WallPublicationTerrain>` |
//! | 6 | `Res<PublicWalls>`（`clamp_wall_height` 0x140ac7f30 的 r8） | 同 |
//! | 7 | `Query<&Roof>`（`clamp_wall_height` 的 r9） | 同 |
//! | 8 | `Res<GameTime>`（`delta` 作速率除数） | 同 |
//!
//! 逐事件行为（`mode` 为事件 `+0x10`）：
//! 1. `mode == 0` → 跳过；`mode != 1` → `todo!()`（原版 `core::panicking::panic("not yet implemented")`，
//!    0x142adf330，长度 0x13）。
//! 2. 事件墙在 `InnerWalls` 里缺失 → `Option::unwrap` 失败 panic（原版首个 hash 探测末尾
//!    `unwrap_failed(0x142adf2e8)`）。
//! 3. `clamp_wall_height`：`PublicWalls` 为空或没有该墙记录 → 返回 `None`（`XOR EAX,EAX`，
//!    0x140ac7ffe），**不是 panic**，此时使用事件原值；有记录则先断言 `count >= 2 && length > 0`
//!    （`PublicWalls::get` 同式 panic）再按屋顶/特殊标志夹取。
//! 4. 夹取后值 `==` 当前高度（有序比较，NaN 同样跳过）→ 跳过该事件。
//! 5. 锚点：事件带锚点则用之；否则取源曲线 `u = 0.5` 的点 `(x, 0, z)`（曲线点数 < 2 时原版
//!    `panic_bounds_check`）；再把地形高度加到 `y`。
//! 6. `try_update_state` 写入高度；若曲线类别 `< 2` 且事件 `+0x20` 字节为 0，则**第二次**写入同一高度，
//!    并向 `PlayBuildingAudio` 写 `{ id: wall, position: anchor, editing_speed: 速率归一化, kind: 3 }`；
//!    否则向 `RequestWallMaterialAudio` 写 `{ id: wall, position: anchor, kind: 0, flag }`
//!    （曲线类别 `< 2` 时 flag 固定为 1，其余沿用事件字节）。
//!
//! 数值与分支细节由 [`apply_resolved_height_command`]（`systems::wall::height_command`）承担；本函数只做
//! 原版系统边界上的参数面、`PublicWalls` 缺失处理与两个事件队列的映射。

use crate::resources::roofs::Roof;
use crate::resources::time::GameTime;
use crate::resources::walls::InnerWalls;
use crate::resources::walls::WallId;
use crate::resources::walls::public_walls::PublicWalls;
use crate::systems::audio::{PlayBuildingAudio, RequestWallMaterialAudio};
use crate::systems::wall::adjust_wall_height::AdjustWallHeightCmd;
use crate::systems::wall::height_command::ResolvedWallHeightClamp;
use crate::systems::wall::height_command::WallHeightFeedback;
use crate::systems::wall::height_command::apply_resolved_height_command;
use crate::systems::wall::wall_state::publish_public_wall_state::WallPublicationTerrain;
use bevy_ecs::prelude::*;
use glam::Vec3;

/// `clamp_wall_height`（0x140ac7f30）前半：`PublicWalls` 没有该墙 → `None`。
fn resolve_clamp(
    public: &PublicWalls,
    wall: WallId,
    roofs: &Query<&Roof>,
) -> Option<ResolvedWallHeightClamp> {
    // `get` 内含 `assertion failed: w.simplified_curve_2d.is_valid()`。
    let state = public.get(wall)?;
    Some(ResolvedWallHeightClamp {
        roof_height_01: public
            .roof_component(wall, |entity| roofs.get(entity).ok())
            .map(Roof::height_01),
        special_flag: state.authority.source_flag_65(),
    })
}

/// 两个事件队列的映射（原版 0x140ac24e1..0x140ac2630 的两处 push）。
fn emit_feedback(
    feedback: WallHeightFeedback,
    material: &mut EventWriter<RequestWallMaterialAudio>,
    building: &mut EventWriter<PlayBuildingAudio>,
) {
    match feedback {
        WallHeightFeedback::Building {
            wall,
            anchor_world,
            observed_kind,
            intensity,
        } => {
            building.write(PlayBuildingAudio {
                id: wall.0,
                position: Vec3::from_array(anchor_world),
                editing_speed: intensity,
                kind: observed_kind,
            });
        }
        WallHeightFeedback::WallMaterial {
            wall,
            anchor_world,
            observed_kind,
            observed_flag,
        } => {
            material.write(RequestWallMaterialAudio {
                id: wall.0,
                position: Vec3::from_array(anchor_world),
                kind: observed_kind,
                flag: observed_flag,
            });
        }
    }
}

pub fn adjust_wall_height_impl(
    mut requests: EventReader<AdjustWallHeightCmd>,
    mut inner: ResMut<InnerWalls>,
    mut material_audio: EventWriter<RequestWallMaterialAudio>,
    mut building_audio: EventWriter<PlayBuildingAudio>,
    terrain: Res<WallPublicationTerrain>,
    public: Res<PublicWalls>,
    roofs: Query<&Roof>,
    time: Res<GameTime>,
) {
    for event in requests.read() {
        let request = &event.0;
        if request.mode == 0 {
            continue;
        }
        if request.mode != 1 {
            todo!();
        }
        let wall = WallId(request.wall.0);
        let Some(state) = inner.get(wall) else {
            panic!("called `Option::unwrap()` on a `None` value (adjust_wall_height_impl: wall {wall:?} missing from InnerWalls)");
        };
        let clamp = resolve_clamp(&public, wall, &roofs);
        let mut height = state.height();
        let outcome = match apply_resolved_height_command(
            request,
            &mut height,
            state.source().curve_kind_tag,
            &state.source().curve,
            &terrain.0,
            clamp,
            time.delta,
        ) {
            Ok(outcome) => outcome,
            // 原版没有错误事件：`mode` 已在上方处理，曲线点数不足走 `panic_bounds_check`。
            Err(error) => panic!("adjust_wall_height_impl: {error:?} for wall {wall:?}"),
        };
        let Some(outcome) = outcome else {
            continue;
        };
        for update in outcome.authority_updates {
            inner.update_wall_state_height(wall, update.requested_height);
        }
        emit_feedback(outcome.feedback, &mut material_audio, &mut building_audio);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::resources::roofs::geometry::surface::Curve2;
    use crate::resources::walls::WallState;
    use crate::resources::walls::publication_source::TerrainHeightGrid512;
    use crate::systems::wall::height_command::AdjustWallHeightRequest;
    use bevy_ecs::system::RunSystemOnce;

    fn world_with_wall(kind_height: f32) -> (World, WallId) {
        let mut world = World::new();
        crate::systems::wall::adjust_wall_height::register_wall_height_events(&mut world);
        world.insert_resource(GameTime {
            delta: 0.5,
            ..Default::default()
        });
        world.insert_resource(WallPublicationTerrain(TerrainHeightGrid512 {
            encoded: vec![0u16; 512 * 512].into(),
            minimum_y: 0.0,
            maximum_y: 4.0,
            sampling_fix: true,
        }));
        world.insert_resource(PublicWalls::default());
        let curve = Curve2::try_new(vec![[0.0, 0.0], [4.0, 0.0]], false).unwrap();
        let mut inner = InnerWalls::new();
        // 自由手墙（类别 tag 2）：走 `RequestWallMaterialAudio` 分支。
        let id = inner.new_wall(WallState::new(curve, kind_height, 0, 0, false));
        world.insert_resource(inner);
        (world, id)
    }

    fn send(world: &mut World, request: AdjustWallHeightRequest) {
        world
            .resource_mut::<Events<AdjustWallHeightCmd>>()
            .send(AdjustWallHeightCmd(request));
    }

    fn request(wall: WallId, mode: u32, value: f32, flag: u8) -> AdjustWallHeightRequest {
        AdjustWallHeightRequest {
            anchor: None,
            mode,
            value,
            wall: crate::resources::walls::publication_fields::WallId(wall.0),
            observed_audio_flag: flag,
        }
    }

    fn material_events(world: &World) -> Vec<RequestWallMaterialAudio> {
        let events = world.resource::<Events<RequestWallMaterialAudio>>();
        events.get_cursor().read(events).copied().collect()
    }

    fn building_events(world: &World) -> Vec<PlayBuildingAudio> {
        let events = world.resource::<Events<PlayBuildingAudio>>();
        events.get_cursor().read(events).copied().collect()
    }

    #[test]
    fn mode_zero_is_skipped_without_touching_anything() {
        let (mut world, id) = world_with_wall(2.0);
        send(&mut world, request(id, 0, 5.0, 0));
        world.run_system_once(adjust_wall_height_impl).unwrap();
        assert_eq!(world.resource::<InnerWalls>().get_unwrapped(id).height(), 2.0);
        assert!(material_events(&world).is_empty());
        assert!(building_events(&world).is_empty());
    }

    #[test]
    #[should_panic(expected = "not yet implemented")]
    fn unsupported_mode_is_the_original_todo_panic() {
        let (mut world, id) = world_with_wall(2.0);
        send(&mut world, request(id, 2, 5.0, 0));
        world.run_system_once(adjust_wall_height_impl).unwrap();
    }

    #[test]
    #[should_panic(expected = "missing from InnerWalls")]
    fn missing_inner_wall_panics_like_option_unwrap() {
        let (mut world, _) = world_with_wall(2.0);
        send(&mut world, request(WallId(9999), 1, 5.0, 0));
        world.run_system_once(adjust_wall_height_impl).unwrap();
    }

    /// 无 `PublicWalls` 记录 → `clamp_wall_height` 返回 `None`，事件原值直接生效
    /// （超出夹取上限 14.0 的值也不会被夹）。
    #[test]
    fn missing_public_record_uses_the_unclamped_event_value() {
        let (mut world, id) = world_with_wall(2.0);
        send(&mut world, request(id, 1, 20.0, 1));
        world.run_system_once(adjust_wall_height_impl).unwrap();
        assert_eq!(world.resource::<InnerWalls>().get_unwrapped(id).height(), 20.0);
        let material = material_events(&world);
        assert_eq!(material.len(), 1);
        assert_eq!(material[0].id, id.0);
        assert_eq!(material[0].kind, 0);
        assert_eq!(material[0].flag, 1);
        // 锚点：曲线 u=0.5 的点 (2, 0, 0)，y 加地形高度（全 0 编码 → minimum_y = 0）。
        assert_eq!(material[0].position, Vec3::new(2.0, 0.0, 0.0));
        assert!(building_events(&world).is_empty());
    }

    #[test]
    fn equal_height_is_skipped() {
        let (mut world, id) = world_with_wall(2.0);
        send(&mut world, request(id, 1, 2.0, 0));
        world.run_system_once(adjust_wall_height_impl).unwrap();
        assert!(material_events(&world).is_empty());
        assert!(building_events(&world).is_empty());
    }

    #[test]
    fn explicit_anchor_is_kept_and_terrain_is_added() {
        let (mut world, id) = world_with_wall(2.0);
        let mut r = request(id, 1, 3.0, 1);
        r.anchor = Some([1.0, 0.5, 2.0]);
        send(&mut world, r);
        world.run_system_once(adjust_wall_height_impl).unwrap();
        let material = material_events(&world);
        assert_eq!(material.len(), 1);
        assert_eq!(material[0].position, Vec3::new(1.0, 0.5, 2.0));
    }

    /// 曲线类别 `< 2` 且事件字节为 0：`PlayBuildingAudio { kind: 3, editing_speed }`，
    /// 速率 `|prev - new| / delta` 在 `0..100` 上归一化后夹到 `[0, 1]`。
    #[test]
    fn regular_wall_with_clear_flag_writes_building_audio() {
        let (mut world, id) = world_with_wall(2.0);
        let shape = {
            let inner = world.resource::<InnerWalls>();
            let mut bytes = *inner.get_unwrapped(id).source().shape.bytes();
            bytes[..4].copy_from_slice(&1u32.to_le_bytes());
            crate::resources::walls::publication_fields::WallShapeSnapshot::from_observed(bytes)
        };
        assert!(world.resource_mut::<InnerWalls>().update_wall_state_enclosure(id, shape));
        send(&mut world, request(id, 1, 3.0, 0));
        world.run_system_once(adjust_wall_height_impl).unwrap();
        assert_eq!(world.resource::<InnerWalls>().get_unwrapped(id).height(), 3.0);
        assert!(material_events(&world).is_empty());
        let building = building_events(&world);
        assert_eq!(building.len(), 1);
        assert_eq!(building[0].id, id.0);
        assert_eq!(building[0].kind, 3);
        // |2 - 3| / 0.5 = 2 -> 2 / 100.
        assert!((building[0].editing_speed - 0.02).abs() < 1e-6);
    }

    /// 曲线类别 `< 2` 但事件字节非 0：走 `RequestWallMaterialAudio`，flag 固定为 1。
    #[test]
    fn regular_wall_with_set_flag_writes_material_audio_with_flag_one() {
        let (mut world, id) = world_with_wall(2.0);
        let shape = {
            let inner = world.resource::<InnerWalls>();
            let mut bytes = *inner.get_unwrapped(id).source().shape.bytes();
            bytes[..4].copy_from_slice(&1u32.to_le_bytes());
            crate::resources::walls::publication_fields::WallShapeSnapshot::from_observed(bytes)
        };
        world.resource_mut::<InnerWalls>().update_wall_state_enclosure(id, shape);
        send(&mut world, request(id, 1, 3.0, 7));
        world.run_system_once(adjust_wall_height_impl).unwrap();
        let material = material_events(&world);
        assert_eq!(material.len(), 1);
        assert_eq!(material[0].flag, 1);
        assert!(building_events(&world).is_empty());
    }
}
