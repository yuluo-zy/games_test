//! process_autoclutter_candidates（B 主基线；0x1422ce260，size 2375）。
//!
//! fresh 反编译（2026-10-17，--worker shared）控制流：
//! 1. 读 state+0x20/+0x28 区域候选（stride 0x98）；
//! 2. `from_iter<OrderedCandidate, FilterMap<Enumerate<Iter>, closure_env_0>>`（L40）枚举+过滤
//!    （带 u16 源下标 @+0xc）；
//! 3. 排序（closure_env_1 比较器；<0x15 用 insertion_sort_shift_left 否则 driftsort_main）；
//! 4. 遍历有序候选：源候选 `byte@+0x30 == 6` → 强制接受；否则对已接受缓冲（stride 0x20，
//!    存源候选 +0x68..+0x80 Rectangle3d）逐条 `Rectangle3d::overlaps_rectangle3d`
//!    （0x140c98580）剔除重叠；
//! 5. 接受路径 switch 源判别 `*puVar26`（3/4/5）：case3 截 u32、case4 Arc LOCK++、case5
//!    裸指针；复制字段写输出 state+0x48（stride 0x68，grow_one<WallPathSegment>）；
//! 6. debug `*(in_R9+0xf2)==1 && *(in_R9+0x102)==4` 时 DebugRectangle3d 可视化。
//!
//! 源(0x98)→输出(0x68)字段映射（O）：
//! +0x00 tag；+0x08 Arc/数据指针（case4 LOCK++、case5 源指针）；+0x10 键（case3/默认截 u32、
//! case4 全 u64）；+0x18=源+0x18；+0x20=源+0x20；+0x28=源+0x28；+0x30=源+0x30 byte；
//! +0x34=源+0x34；+0x38=源+0x38；+0x40=源+0x40；+0x48=源+0x48；+0x50=源+0x50；
//! +0x54=源+0x54 低 32；+0x58=源+0x54 高 32；+0x5c=源+0x5c；+0x60=源+0x60。

use bevy_ecs::prelude::*;
use country_core::{
    resources::gui::{DebugGui, DebugTriggerProxyVis},
    systems::ui::signifiers::UiSignifierStream,
    systems::ui_clutter::DeletedAutoClutter,
    utils::debug_viz::{DebugColor, DebugRectangle3d},
};

use super::{
    AutoClutterCandidate, AutoClutterPayload, AutoClutterSpawn, AutoClutterState,
    ResolvedAutoClutterObject,
};

/// 有序候选（16 B；u16 源下标 @+0xc）。
#[derive(Clone, Copy, Debug)]
struct OrderedCandidate {
    /// +0x00：键/哈希（enumerate 上下文，sort 后遍历不直接读）。
    region_key: u64,
    priority: u32,
    /// +0x0c：u16 源候选下标。
    index: u16,
}

/// 已接受缓冲（stride 0x20；Rectangle3d @+0x00 起）。
#[derive(Clone, Copy, Debug)]
struct AcceptedRecord {
    rect: utils::geometry::rectangle::Rectangle3d,
}

/// process_autoclutter_candidates（0x1422ce260）。
pub fn process_autoclutter_candidates(
    mut state: ResMut<AutoClutterState>,
    mut signifiers: ResMut<UiSignifierStream>,
    deleted: Res<DeletedAutoClutter>,
    debug_gui: Res<DebugGui>,
) {
    // 1+2+3：枚举过滤（closure$0 L40）→ 稳定排序（closure$1 L56 比较器）。
    let AutoClutterState {
        region_candidates,
        spawn_candidates,
        ..
    } = &mut *state;
    let mut ordered: Vec<OrderedCandidate> = region_candidates
        .iter()
        .enumerate()
        // B@1422ce3f4 closure$0 filters on the candidate's native +0x10 key.
        // +0x40 is a separate source payload word and is not the generated ID.
        .filter_map(|(i, c)| (!deleted.is_deleted(c.key)).then_some(OrderedCandidate {
            region_key: c.region_key, priority: c.priority, index: i as u16,
        }))
        .collect();
    // B 141cf9b50：优先级、区域键均降序，相等时维持原先顺序。
    ordered.sort_by(|a, b| b.priority.cmp(&a.priority).then_with(|| b.region_key.cmp(&a.region_key)));

    // O：accepted 缓冲按源候选数预分配；返回时先释放 accepted 再释放 ordered。
    let mut accepted: Vec<AcceptedRecord> = Vec::with_capacity(region_candidates.len());
    for oc in &ordered {
        // O：`panic_bounds_check`（len <= 源下标）。
        assert!(
            (oc.index as usize) < region_candidates.len(),
            "process_autoclutter_candidates: bounds check failed"
        );
        let candidate = &region_candidates[oc.index as usize];
        let mut accepted_spawn = None;
        if candidate.forced() {
            // 强制接受路径（byte@+0x30==6）：复制字段。
            accepted_spawn = Some(accept_candidate(candidate, &mut accepted));
        } else {
            // 贪心矩形重叠剔除（O：Rectangle3d::overlaps_rectangle3d）。
            let mut overlap = false;
            for acc in &accepted {
                if acc.rect.overlaps_rectangle3d(candidate.rect) {
                    overlap = true;
                    break;
                }
            }
            if !overlap {
                accepted_spawn = Some(accept_candidate(candidate, &mut accepted));
            }
        }
        let accepted = accepted_spawn.is_some();
        if let Some(spawn) = accepted_spawn {
            spawn_candidates.push(spawn);
        }
        if debug_gui.show_trigger_proxies
            && debug_gui.trigger_proxy_visualization == DebugTriggerProxyVis::ClutterCollisions
        {
            // O@1422ce6ef..ce7c: every examined candidate is emitted after the
            // accept/reject decision. Native color 0 is accepted, color 1 is
            // rejected; the constructor leaves width at its native default 1.
            DebugRectangle3d::new(debug_rectangle_array(candidate.rect))
                .color(if accepted {
                    DebugColor::White
                } else {
                    DebugColor::Red
                })
                .show(&mut signifiers);
        }
    }
}

fn debug_rectangle_array(
    rect: utils::geometry::rectangle::Rectangle3d,
) -> [f32; 8] {
    [
        rect.footprint.axis.x,
        rect.footprint.axis.y,
        rect.footprint.center.x,
        rect.footprint.center.y,
        rect.footprint.size.x,
        rect.footprint.size.y,
        rect.bottom,
        rect.top,
    ]
}

/// 接受路径（O：switch 源判别 3/4/5 + 写已接受缓冲 + 输出字段复制）。
fn accept_candidate(
    c: &AutoClutterCandidate,
    accepted: &mut Vec<AcceptedRecord>,
) -> AutoClutterSpawn {
    // O@1422ce677..ce6cc appends the source Rectangle3d before the source
    // union clone switch (including tag-4 Arc strong increment).
    accepted.push(AcceptedRecord {
        rect: c.rect,
    });
    // Production candidates are complete before acceptance. In particular,
    // never manufacture an Arc or a prefab name from a legacy numeric field.
    let object: ResolvedAutoClutterObject = c
        .resolved
        .clone()
        .unwrap_or_else(|| panic!(
            "autoclutter candidate is missing its resolved source/mesh owner (kind={}, key={:#x})",
            c.kind, c.key
        ));
    assert_eq!(
        object.source.native_tag(),
        c.kind,
        "autoclutter source discriminant mismatch"
    );
    AutoClutterSpawn {
        kind: c.kind,
        payload: AutoClutterPayload::Object(object),
        f18: (c.f18 & 0xffff_ffff) as u32,
        // B@1422ce807..ce860 copies candidate +0x10 into output +0x10.
        // The source union switch only changes whether +0x10 is truncated
        // (tag 3/default) or retained as the full u64 (tag 4).
        key: match c.kind {
            // Tag 3 explicitly projects candidate +0x10 to u32. Tags 4
            // and 5 retain the full candidate ID; only the source owner
            // representation differs.
            3 => u64::from(c.key as u32),
            _ => c.key,
        },
        marker: c.forced,
        selector_value: c.f34,
        // B 输出 +0x48..+0x53：原 candidate 的完整 xyz。
        position: glam::Vec3::new(
            f32::from_bits(c.f48 as u32),
            f32::from_bits((c.f48 >> 32) as u32),
            f32::from_bits(c.f50),
        ),
        host_id: c.f48,
        f48: c.f48,
        f50: c.f50 as u64,
        f54: (c.f54 & 0xffff_ffff) as u32,
        f58: ((c.f54 >> 32) & 0xffff_ffff) as u32,
        f5c: c.f5c,
        scale: f32::from_bits(c.f60),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stale_region_removes_grid_links_and_its_candidate() {
        use crate::autoclutter::{AutoClutterRegion, ClutterCandidateIndex, ClutterRegionId};
        let key = ClutterRegionId(91);
        let mut state = AutoClutterState { lazy_grid: vec![vec![]; 4096], epoch: 4, ..Default::default() };
        let region = AutoClutterRegion { key, epoch: 3, center: glam::Vec3::ZERO,
            basis: [glam::Vec2::X, glam::Vec2::ONE], ..Default::default() };
        state.add_region(key, &region);
        state.regions.get_mut(&key).unwrap().epoch = 3;
        state.regions.get_mut(&key).unwrap().candidate_index = ClutterCandidateIndex(0);
        state.region_candidates.push(AutoClutterCandidate { region_key: key.0, ..Default::default() });
        assert!(state.lazy_grid.iter().any(|cell| cell.contains(&key.0)));
        state.remove_stale_regions();
        state.flush_candidates_pending_removal();
        assert!(state.regions.is_empty());
        assert!(state.region_candidates.is_empty());
        assert!(state.lazy_grid.iter().all(|cell| !cell.contains(&key.0)));
    }

    #[test]
    fn replacing_a_lazy_grid_region_keeps_exactly_one_link_per_cell() {
        use crate::autoclutter::{AutoClutterRegion, ClutterRegionId};
        let key = ClutterRegionId(0x1234);
        let mut state = AutoClutterState::default();
        let first = AutoClutterRegion {
            key,
            center: glam::Vec3::new(-3.0, 1.0, 2.0),
            basis: [glam::Vec2::X, glam::Vec2::new(3.0, 2.0)],
            ..Default::default()
        };
        let second = AutoClutterRegion {
            center: glam::Vec3::new(5.0, 4.0, -6.0),
            basis: [glam::Vec2::Y, glam::Vec2::new(2.0, 4.0)],
            ..first.clone()
        };
        state.add_region(key, &first);
        state.add_region(key, &second);
        assert!(state.lazy_grid.iter().all(
            |cell| cell.iter().filter(|global| **global == key.0).count() <= 1
        ));

        let aabb = second.rectangle2d().aabb();
        let (minimum, end) = crate::autoclutter::read_bush_positions_from_gpu::bush_rect_cells(
            aabb.center() - aabb.half_dims(),
            aabb.center() + aabb.half_dims(),
        );
        for y in minimum[1]..end[1] {
            for x in minimum[0]..end[0] {
                let cell = y as usize * 64 + x as usize;
                assert_eq!(
                    state.lazy_grid[cell]
                        .iter()
                        .filter(|global| **global == key.0)
                        .count(),
                    1
                );
            }
        }
    }

    #[test]
    fn dirty_and_stale_scratch_reuse_capacity_across_frames() {
        use crate::autoclutter::{AutoClutterRegion, ClutterCandidateIndex, ClutterRegionId};
        let dirty_key = ClutterRegionId(7);
        let mut state = AutoClutterState {
            lazy_grid: vec![vec![dirty_key.0]],
            epoch: 3,
            ..Default::default()
        };
        state.regions.insert(
            dirty_key,
            AutoClutterRegion {
                key: dirty_key,
                epoch: 3,
                candidate_index: ClutterCandidateIndex(11),
                ..Default::default()
            },
        );
        state.mark_dirty_scratch.reserve_exact(8);
        let dirty_capacity = state.mark_dirty_scratch.capacity();
        state.mark_dirty(0);
        assert_eq!(state.mark_dirty_scratch.capacity(), dirty_capacity);
        state.regions.get_mut(&dirty_key).unwrap().candidate_index = ClutterCandidateIndex(12);
        state.mark_dirty(0);
        assert_eq!(state.mark_dirty_scratch.capacity(), dirty_capacity);
        assert_eq!(state.pending_indices, [11, 12]);

        state.stale_region_scratch.reserve_exact(8);
        let stale_capacity = state.stale_region_scratch.capacity();
        state.regions.clear();
        for id in 20..25 {
            let key = ClutterRegionId(id);
            state.regions.insert(
                key,
                AutoClutterRegion {
                    key,
                    epoch: 2,
                    basis: [glam::Vec2::X, glam::Vec2::ONE],
                    ..Default::default()
                },
            );
        }
        state.remove_stale_regions();
        assert_eq!(state.stale_region_scratch.capacity(), stale_capacity);
        assert!(state.regions.is_empty());
    }

    #[test]
    fn deleted_keys_are_filtered_before_priority_order_and_full_pose_is_copied() {
        use country_core::components::drawable::{
            AutoClutterDestination, AutoClutterSource, AutoClutterSourceIdTag,
        };
        use country_core::name_hash::NameHash;
        let mut world = World::new();
        let candidate = |id, region_key, priority, x: f32| {
            let source = AutoClutterSource::Id {
                tag: AutoClutterSourceIdTag::Native3,
                native_u64_08: id,
                native_u32_10: 0,
            };
            AutoClutterCandidate {
                resolved: Some(super::super::ResolvedAutoClutterObject {
                    metadata: super::super::ResolvedClutterMesh {
                        mesh_name: "test/autoclutter",
                        bounds: utils::geometry::rectangle::Rectangle3d::new(
                            glam::Vec3::ZERO,
                            glam::Vec3::ONE,
                            glam::Vec2::X,
                        ),
                        name_hash: NameHash(id),
                        radius_xz: 1.0,
                        native_group: 0,
                        solid_texture: None,
                    },
                    source,
                    destination: AutoClutterDestination::Terrain,
                }),
                kind: 3,
                key: id,
                f48: u64::from(x.to_bits()) | (u64::from(2.0f32.to_bits()) << 32),
                f50: 3.0f32.to_bits(),
                forced: 6,
                region_key,
                priority,
                f60: 1.0f32.to_bits(),
                ..Default::default()
            }
        };
        world.insert_resource(AutoClutterState {
            region_candidates: vec![candidate(0x1_0000_0001, 1, 2, 1.0),
                candidate(7, 9, 2, 4.0), candidate(8, 2, 3, 5.0),
                candidate(9, 3, 9, 6.0)],
            ..Default::default()
        });
        world.init_resource::<UiSignifierStream>();
        world.init_resource::<DebugGui>();
        let mut deleted = DeletedAutoClutter::default();
        deleted.0.insert(9);
        world.insert_resource(deleted);
        let mut schedule = Schedule::default();
        schedule.add_systems(process_autoclutter_candidates);
        schedule.run(&mut world);
        let output = &world.resource::<AutoClutterState>().spawn_candidates;
        assert_eq!(output.iter().map(|s| s.key).collect::<Vec<_>>(), [8, 7, 1]);
        assert_eq!(output[2].position, glam::Vec3::new(1.0, 2.0, 3.0));
    }

    #[test]
    fn debug_collision_rectangles_follow_sort_order_and_acceptance() {
        use country_core::components::drawable::{AutoClutterDestination, AutoClutterSource,
            AutoClutterSourceIdTag};
        use country_core::name_hash::NameHash;
        use country_core::systems::ui::signifiers::signifier_create::UiSignifierEntry;

        let candidate = |id, priority, x: f32| AutoClutterCandidate {
            resolved: Some(super::super::ResolvedAutoClutterObject {
                metadata: super::super::ResolvedClutterMesh {
                    mesh_name: "test/autoclutter",
                    bounds: utils::geometry::rectangle::Rectangle3d::new(
                        glam::Vec3::ZERO,
                        glam::Vec3::ONE,
                        glam::Vec2::X,
                    ),
                    name_hash: NameHash(id),
                    radius_xz: 1.0,
                    native_group: 0,
                    solid_texture: None,
                },
                source: AutoClutterSource::Id {
                    tag: AutoClutterSourceIdTag::Native3,
                    native_u64_08: id,
                    native_u32_10: 0,
                },
                destination: AutoClutterDestination::Terrain,
            }),
            kind: 3,
            key: id,
            f48: u64::from(x.to_bits()) | (u64::from(0.0f32.to_bits()) << 32),
            f50: 0.0f32.to_bits(),
            f60: 1.0f32.to_bits(),
            rect: utils::geometry::rectangle::Rectangle3d::new(
                glam::Vec3::new(x, 0.0, 0.0),
                glam::Vec3::ONE,
                glam::Vec2::X,
            ),
            region_key: id,
            priority,
            ..Default::default()
        };

        let mut world = World::new();
        world.insert_resource(AutoClutterState {
            region_candidates: vec![candidate(1, 1, 0.0), candidate(2, 0, 0.0)],
            ..Default::default()
        });
        world.init_resource::<DeletedAutoClutter>();
        world.init_resource::<UiSignifierStream>();
        world.insert_resource(DebugGui {
            show_trigger_proxies: true,
            trigger_proxy_visualization: DebugTriggerProxyVis::ClutterCollisions,
            ..DebugGui::default()
        });
        let mut schedule = Schedule::default();
        schedule.add_systems(process_autoclutter_candidates);
        schedule.run(&mut world);

        let entries = world.resource_mut::<UiSignifierStream>().take_entries();
        assert_eq!(entries.len(), 12);
        let names: Vec<_> = entries
            .iter()
            .map(|entry| match entry {
                UiSignifierEntry::Factory(factory) => factory.name().unwrap(),
                UiSignifierEntry::Raw(_) => panic!("debug rectangles must use real factories"),
            })
            .collect();
        assert!(names.iter().take(6).all(|name| name.contains("White")));
        assert!(names.iter().skip(6).all(|name| name.contains("Red")));
    }
}
