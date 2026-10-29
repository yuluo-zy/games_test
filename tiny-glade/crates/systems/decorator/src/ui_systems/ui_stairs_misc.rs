//! B `ui_systems/ui_stairs_misc.rs` — 楼梯交互杂项（O 机器码手解）。
//!
//! - `stairs_reset_for_new_frame` 0x14079c350：先标记StairsState已变更；
//!   仅编辑楼梯点或放置楼梯时保留端点，其余模式调用原unset helper。
//! - `continue_stairs` 0x14079c3a0：实际 AppMode 编辑楼梯或放置楼梯时，
//!   读取活 DeferredDecoratorOpEvents 的0x88B记录；连接原端点与新节点，
//!   `[@+0x80]==0` 时用原16B地址和节点ID设置新端点。
use bevy_ecs::prelude::*;
use country_core::resources::stairs::stairs_state::{ObservedStairEndpoint, StairsState};

use country_core::systems::mode_manager::{AppMode, DecoratorType, EditSubject};

/// B 0x14079c350（O）：每帧楼梯状态复位。
pub fn stairs_reset_for_new_frame(
    mode: Res<AppMode>,
    mut stairs: ResMut<StairsState>,
) {
    // 原前四条指令写ResMut的changed tick，即使保留端点也必须标记。
    stairs.set_changed();
    if !matches!(
        *mode,
        AppMode::Editing(EditSubject::StairPoint { .. })
            | AppMode::Decorator(DecoratorType::Stairs)
    ) {
        // 原helper只写端点首个u64为3，保留其余16B与图状态。
        stairs.unset_endpoint();
    }
}

#[cfg(test)]
mod stairs_reset_native_tests {
    use super::*;
    use bevy_ecs::system::IntoSystem;
    use country_core::systems::mode_manager::{EraserLayer, WallShape};

    #[test]
    fn complete_reset_matches_original_endpoint_and_change_tracking() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-root-stairs-reset-native.json"
        )))
        .unwrap();
        let decode = |s: &str| -> [u8; 24] {
            std::array::from_fn(|i| u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap())
        };
        let mut checked = 0;
        for case in fixture["cases"].as_array().unwrap() {
            let tag = case["mode"].as_u64().unwrap() as u32;
            let payload = case["payload"].as_u64().unwrap() as u8;
            let mode = match tag {
                0 => AppMode::Editing(EditSubject::None),
                1 => AppMode::Editing(EditSubject::Wall(99)),
                2 => AppMode::Editing(EditSubject::Roof(99)),
                3 => AppMode::Editing(EditSubject::DecoClutter { decorator_id: 99 }),
                4 => AppMode::Editing(EditSubject::StairPoint { decorator_id: 99 }),
                5 => AppMode::Editing(EditSubject::Terrain(99)),
                6 => AppMode::Wall(WallShape::Circle),
                7 => AppMode::Path,
                8 => AppMode::Cobblestone(99),
                9 => AppMode::Eraser(EraserLayer::All),
                10 => AppMode::Terrain,
                11 => {
                    // 原件含非法Decorator枚举；Rust侧仅构造合法业务输入。
                    let Some(kind) = DecoratorType::from_native_tag(payload) else { continue };
                    AppMode::Decorator(kind)
                }
                12 => AppMode::Color,
                13 => AppMode::Replay,
                14 => AppMode::LoadGame,
                15 => AppMode::Water,
                16 => AppMode::Tree,
                17 => AppMode::Garden,
                18 => AppMode::Fences,
                19 => AppMode::Photomode,
                20 => AppMode::CloneClutter,
                _ => unreachable!(),
            };
            let endpoint = ObservedStairEndpoint::from_observed(decode(case["before"].as_str().unwrap()));
            let mut stairs = StairsState::empty(endpoint);
            stairs.add_edge(7, 19);
            stairs.make_backup();
            let mut world = World::new();
            world.insert_resource(mode);
            world.insert_resource(stairs);
            let mut system = IntoSystem::into_system(stairs_reset_for_new_frame);
            system.initialize(&mut world);
            world.clear_trackers();
            system.run((), &mut world);
            let stairs = world.resource_ref::<StairsState>();
            assert_eq!(stairs.endpoint().observed(), &decode(case["after"].as_str().unwrap()), "mode={tag}, payload={payload}");
            assert!(stairs.is_changed());
            assert!(stairs.is_backed_up());
            assert_eq!(stairs.graph().node_count(), 2);
            assert_eq!(stairs.graph().edge_count(), 1);
            assert!(stairs.graph().contains_edge(7, 19));
            // 同值重复复位也必须继续标记changed，不得优化掉原副作用。
            world.clear_trackers();
            system.run((), &mut world);
            assert!(world.resource_ref::<StairsState>().is_changed());
            checked += 1;
        }
        assert_eq!(checked, 123);
    }
}

#[cfg(test)]
mod continue_stairs_tests {
    use super::*;
    use bevy_ecs::system::IntoSystem;
    use country_core::{resources::walls::decorator::DeferredDecoratorOpEvents,
        systems::mode_manager::{AppMode, DecoratorType, EditSubject}};

    fn event(tag: u32, kind: u8, node: u32, applied: bool) -> [u8; 0x88] {
        let mut event = [0; 0x88];
        event[..4].copy_from_slice(&tag.to_le_bytes());
        // 原只检查u32 tag，padding非零也必须处理，不能比较整个u64。
        event[4..8].copy_from_slice(&0xdead_beefu32.to_le_bytes());
        event[8..16].copy_from_slice(&0u64.to_le_bytes());
        event[16..24].copy_from_slice(&0x1234_5678_9abc_def0u64.to_le_bytes());
        event[0x40..0x44].copy_from_slice(&node.to_le_bytes());
        event[0x80] = u8::from(applied);
        event[0x81] = kind;
        event
    }

    fn run(mode: AppMode, records: &[[u8; 0x88]], stairs: StairsState) -> World {
        let mut world = World::new();
        world.insert_resource(mode);
        world.insert_resource(stairs);
        let events = DeferredDecoratorOpEvents::default();
        for record in records { events.send(*record); }
        world.insert_resource(events);
        let mut system = IntoSystem::into_system(continue_stairs);
        system.initialize(&mut world);
        system.run((), &mut world);
        world
    }

    #[test]
    fn active_modes_connect_previous_endpoint_and_keep_the_live_queue() {
        for mode in [AppMode::Editing(EditSubject::StairPoint { decorator_id: 99 }),
            AppMode::Decorator(DecoratorType::Stairs)] {
            let records = [event(3, 63, 7, false), event(3, 63, 8, false),
                event(3, 63, 8, false), event(3, 63, 9, true)];
            let world = run(mode, &records, StairsState::new());
            let stairs = world.resource::<StairsState>();
            assert!(stairs.graph().contains_edge(7, 8));
            assert!(stairs.graph().contains_edge(8, 9));
            assert_eq!(stairs.graph().edge_count(), 2);
            assert_eq!(stairs.endpoint().node(), 8);
            assert_eq!(&stairs.endpoint().observed()[..16], &records[1][8..24]);
            assert_eq!(world.resource::<DeferredDecoratorOpEvents>().with_events(|events| events.len()), 4);
        }
    }

    #[test]
    fn inactive_modes_and_non_stair_records_do_not_touch_the_graph() {
        let record = event(3, 63, 7, false);
        for mode in [AppMode::default(), AppMode::Editing(EditSubject::Terrain(9)),
            AppMode::Decorator(DecoratorType::Lantern), AppMode::Replay, AppMode::Photomode] {
            let world = run(mode, &[record], StairsState::new());
            assert_eq!(world.resource::<StairsState>().graph().node_count(), 0);
            assert!(world.resource::<StairsState>().endpoint().is_unset());
        }
        let world = run(AppMode::Decorator(DecoratorType::Stairs),
            &[event(2, 63, 7, false), event(3, 62, 8, false)], StairsState::new());
        assert_eq!(world.resource::<StairsState>().graph().node_count(), 0);
    }

    #[test]
    fn degree_guard_unsets_endpoint_and_later_records_read_the_updated_endpoint() {
        let world = run(AppMode::Decorator(DecoratorType::Stairs), &[
            event(3, 63, 7, false), event(3, 63, 8, false),
            event(3, 63, 9, true), event(3, 63, 8, false), event(3, 63, 10, false),
        ], StairsState::new());
        let stairs = world.resource::<StairsState>();
        assert_eq!(stairs.graph().edge_count(), 2);
        assert!(stairs.graph().contains_node(10));
        assert_eq!(stairs.endpoint().node(), 10);
        assert_eq!(stairs.graph().neighbors(10).count(), 0);
    }

    #[test]
    fn complete_original_continue_vectors_match_actual_world_state() {
        use country_core::systems::mode_manager::WallShape;
        fn bytes<const N: usize>(hex: &str) -> [u8; N] {
            assert_eq!(hex.len(), N * 2);
            std::array::from_fn(|i| u8::from_str_radix(&hex[i * 2..i * 2 + 2], 16).unwrap())
        }
        fn nodes(value: &str) -> Vec<u32> {
            if value == "-" { vec![] } else { value.split(',').map(|v| v.parse().unwrap()).collect() }
        }
        fn edges(value: &str) -> Vec<(u32, u32)> {
            if value == "-" { return vec![]; }
            value.split(',').map(|value| {
                let (a, b) = value.split_once(':').unwrap();
                (a.parse().unwrap(), b.parse().unwrap())
            }).collect()
        }
        let mut checked = 0;
        let mut unsupported = 0;
        for row in include_str!(concat!(env!("CARGO_MANIFEST_DIR"),
            "/../../../reports/module-recovery-2026-10-23/m01-continue-stairs-native.tsv"))
            .lines().filter(|line| !line.starts_with('#') && !line.is_empty()) {
            let cells: Vec<_> = row.split('\t').collect();
            assert_eq!(cells.len(), 14);
            let tag: u32 = cells[1].parse().unwrap();
            let payload: u8 = cells[2].parse().unwrap();
            let mode = match tag {
                0 => AppMode::default(),
                1 => AppMode::Editing(EditSubject::Wall(0)),
                2 => AppMode::Editing(EditSubject::Roof(0)),
                3 => AppMode::Editing(EditSubject::DecoClutter { decorator_id: 0 }),
                4 => AppMode::Editing(EditSubject::StairPoint { decorator_id: 0 }),
                5 => AppMode::Editing(EditSubject::Terrain(0)),
                6 => AppMode::Wall(WallShape::Circle),
                7 => AppMode::Path,
                10 => AppMode::Terrain,
                11 => {
                    let Some(kind) = DecoratorType::from_native_tag(payload) else {
                        unsupported += 1; continue;
                    };
                    AppMode::Decorator(kind)
                }
                12 => AppMode::Color,
                20 => AppMode::CloneClutter,
                // 原探针另外测试非法Rust枚举判别21/UINT_MAX；宿主不制造UB。
                _ => { unsupported += 1; continue; }
            };
            let mut stairs = StairsState::empty(ObservedStairEndpoint::from_observed(bytes(cells[4])));
            for node in nodes(cells[5]) { stairs.add_node(node); }
            for (a, b) in edges(cells[6]) { stairs.add_edge(a, b); }
            stairs.make_backup();
            let mut world = World::new();
            world.insert_resource(stairs);
            world.insert_resource(mode);
            let events = DeferredDecoratorOpEvents::default();
            if cells[7] != "-" {
                for event in cells[7].split(';') { events.send(bytes(event)); }
            }
            world.insert_resource(events);
            let mut system = IntoSystem::into_system(continue_stairs);
            system.initialize(&mut world);
            world.clear_trackers();
            system.run((), &mut world);
            let stairs = world.resource::<StairsState>();
            let mut actual_nodes: Vec<_> = stairs.graph().nodes().collect();
            actual_nodes.sort_unstable();
            let mut expected_nodes = nodes(cells[9]); expected_nodes.sort_unstable();
            assert_eq!(actual_nodes, expected_nodes, "{} nodes", cells[0]);
            let normalized = |values: Vec<(u32, u32)>| {
                let mut values: Vec<_> = values.into_iter().map(|(a,b)| (a.min(b), a.max(b))).collect();
                values.sort_unstable(); values
            };
            let actual_edges = stairs.graph().all_edges().map(|(a,b,_)| (a,b)).collect();
            assert_eq!(normalized(actual_edges), normalized(edges(cells[10])), "{} edges", cells[0]);
            let expected = bytes::<24>(cells[8]);
            // 原locator末4B是非活跃padding；前20B全部对照原实际写面。
            assert_eq!(&stairs.endpoint().observed()[..20], &expected[..20], "{} endpoint", cells[0]);
            assert_eq!(stairs.is_backed_up(), cells[13] == "1", "{} backup", cells[0]);
            assert_eq!(world.resource_ref::<StairsState>().is_changed(), cells[11] == "4660",
                "{} changed tick", cells[0]);
            checked += 1;
        }
        assert_eq!((checked, unsupported), (317, 7));
    }
}

/// 原 B 14079c3a0：读取活的延迟操作队列，逐项延续当前楼梯端点。
/// 原配置 1400b1230 的三组参数是 AppMode、StairsState、DeferredEvents，
/// 不是派发器或备份队列；读锁覆盖整个遍历，不排空本帧事件。
pub fn continue_stairs(
    mode: Res<country_core::systems::mode_manager::AppMode>,
    mut stairs: ResMut<StairsState>,
    events: Res<country_core::resources::walls::decorator::DeferredDecoratorOpEvents>,
) {
    use country_core::systems::mode_manager::{AppMode, DecoratorType, EditSubject};
    if !matches!(*mode, AppMode::Editing(EditSubject::StairPoint { .. })
        | AppMode::Decorator(DecoratorType::Stairs)) {
        return;
    }
    // 原 14079c3fb..402 在取得队列锁前标记资源 changed，即使队列为空。
    let stairs = &mut *stairs;
    events.with_events(|events| { for event in events {
        if event[0..4] != 3u32.to_le_bytes() || event[0x81] != b'?' {
            continue;
        }
        let node = u32::from_le_bytes(event[0x40..0x44].try_into().unwrap());
        stairs.add_node(node);
        // 原 14079c4d4 取当前 endpoint，事件的存储地址不是上一节点 ID。
        let previous = stairs.endpoint();
        if !previous.is_unset() && previous.node() != node {
            stairs.add_edge(previous.node(), node);
        }
        if event[0x80] == 0 {
            // 原 DecoratorLocator::new 140833240 写16B存储地址和u32 ID。
            // 最后4B是非活跃padding，宿主初始化为0，不读取原未初始化值。
            let mut locator = [0; 24];
            locator[..16].copy_from_slice(&event[8..24]);
            locator[16..20].copy_from_slice(&node.to_le_bytes());
            stairs.try_set_endpoint(ObservedStairEndpoint::from_observed(locator));
        }
    }});
}
