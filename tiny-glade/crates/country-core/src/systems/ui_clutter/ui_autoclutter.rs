//! `ui_autoclutter.rs` — 右键把 auto-clutter 转 deco-clutter（B 0x14099ca60）。
//!
//! 证据：fresh 反编译（restorer-terrain-clutter/jobs/decomp/14099ca60，1400+ 行）。逐语句对照：
//! - 消费点击事件队列（stride 0x10）；对每个 id 查 0xa0B clutter 记录表，`+0x58 == 0`
//!   视为已删除跳过；
//! - 记录 `+0x94` → 0x48 步长类型表；记录 `+0x50` = 子项列表（stride 0x10）；
//! - 逐子项：`uVar37 = *(uint*)(children + 8 + i*0x10)`；父记录 `0x60` 步长（`uVar37*0x60`），
//!   变换记录 `0x30` 步长（`uVar37*0x30`）；
//! - **`parent = puVar19[4]`（父记录 +0x10）**：`HashMap<DecoratorId,
//!   Vec<DisplayChildAutoClutterInfo>>`；父类型 `*puVar19 < 3` 才处理；
//! - `ClutterMeshTy::from` + `clutter_mesh_ty_data` → DisplayChildAutoClutterInfo
//!   （0x60 B：mesh 4 qword @+0x00、transform 6 qword @+0x20、`+0x50` = 变换记录 +0xc）；
//!   已有键 → `Vec::push`（`grow_one<DisplayChildAutoClutterInfo>`）；无键 →
//!   `RawTable::reserve_rehash` + 新建 `Vec`（cap=1/len=1）；
//! - 第二事件队列：命中 d2s 位图 → 直接调用 `handle_rmb_autoclutter`（crate 以产出
//!   [`RmbAutoClutterHit`] 事件承载）；
//! - 尾部模式门：`disc ∈ [6,0x15) → disc-5`，其余 0；接受 `disc==0` 或
//!   `disc==9 ∧ payload 低字节==3`（Bulldoze），随后 `Rapier::raycast`（命中 tag != 2
//!   ∧ 标签字节 0x0c）→ `DecoratorStorage` 探测 → `UiState::add` → 0x60 记录类型
//!   3..5 子跳表 `&DAT_142abbcdc`（UNRECOVERED_JUMPTABLE）。
//!
//! B 完整 FunctionSystem 与真实 producer/consumer 已确认 `InstaCreateDeco` /
//! `InstaCreateStairPoint` 是创建命令；早前以 show_decorator_icon 地址排除它们的结论撤回。宿主实体表（0xa0B/0x60/0x30 记录）由 [`AutoClutterChildTable`]
//! 适配承载，尾部 raycast/UI 注册路径登记待接。
use bevy_ecs::prelude::*;
use hashbrown::HashMap;

use crate::systems::mode_manager::{AppMode, EraserLayer};

use super::rmb_autoclutter::EvRightClickedAutoClutter;

/// B1408b83d0/2748B: exact original name parser, shared by the actual
/// parent/RMB consumers. This does not search caller-modified mesh catalogs.
pub(super) fn clutter_mesh_type_from_name(name: &str) -> crate::systems::mode_manager::ClutterMeshTy {
    let tag = match name {
        "clutter/plant_pot_v1" => 0,
        "clutter/ladder" => 1,
        "clutter/wheel" => 2,
        "clutter/pitchfork" => 3,
        "clutter/shovel" => 4,
        "clutter/box_w_stuff" => 5,
        "clutter/watering_can" => 6,
        "clutter/firewood" => 7,
        "clutter/bucket" => 8,
        "clutter/birdnest" => 9,
        "clutter/bench" => 10,
        "clutter/bench_w_basket" => 11,
        "clutter/bench_short" => 12,
        "clutter/barrel" => 13,
        "clutter/haystack" => 14,
        "clutter/plant_pot_v2" => 15,
        "clutter/birdhouse" => 16,
        "clutter/basket" => 17,
        "clutter/basket_w_food" => 18,
        "clutter/barrel_w_food_basket" => 19,
        "clutter/potato_sack" => 20,
        "clutter/road_sign" => 21,
        "clutter/sunflower" => 22,
        "clutter/cart" => 23,
        "clutter/cart2" => 24,
        "clutter/crates_w_tools" => 25,
        "clutter/crates_w_flowers" => 26,
        "clutter/window_flowerpot" => 27,
        "clutter/window_shutter" => 28,
        "clutter/reed" => 30,
        "clutter/lilypad" => 31,
        "fallen_leaves" => 32,
        "fallen_petals" => 33,
        "olden_glade_rock" => 34,
        "tree_log" => 35,
        "tree_stump" => 36,
        "clutter/pumpkin" => 37,
        "clutter/pumpkin2" => 38,
        "clutter/pumpkin_w_face" => 39,
        "clutter/cart_w_pumpkins" => 40,
        "clutter/crates_w_pumpkins" => 41,
        "clutter/barrel_w_candles" => 43,
        "clutter/anvil" => 44,
        "clutter/fabric_rolls" => 45,
        "clutter/stall_bread" => 46,
        "clutter/stall_veggies" => 47,
        "clutter/crate_fish" => 48,
        "clutter/rack_brooms" => 49,
        "clutter/flowerbox" => 50,
        "clutter/door_bell" => 51,
        "clutter/door_krans" => 52,
        "clutter/door_krans_autumn" => 53,
        "clutter/cart_w_pumpkins_regular" => 54,
        "clutter/crates_w_pumpkins_regular" => 55,
        _ => unreachable!(),
    };
    crate::systems::mode_manager::ClutterMeshTy::from_native_tag(tag)
        .expect("original ClutterMeshTy name arm has a valid tag")
}

/// Original B InstaCreateDeco: an actual creation command, not the right-click
/// input. Native EventId and inactive extra/padding bytes are not business data.
#[derive(Event, Debug)]
pub struct InstaCreateDeco {
    /// Existing source component's 24-byte destination union. Semantic decoding
    /// remains with placement; never synthesize it from an auto-clutter ID.
    pub destination: [u64; 3],
    pub extra: Option<crate::resources::walls::decorator::DecoratorExtraInfo>,
    pub rotation: f32,
    pub position: glam::Vec3,
    pub ty: crate::systems::mode_manager::ClutterMeshTy,
}

/// Original B fixed-size stair creation business value. Its typed native Events
/// destructor releases only two 120-byte-stride buffers, with no element/Arc/Vec
/// destruction. No native pointer is converted to a Rust reference or owner.
/// Destination enums and seven color bytes still need semantic recovery; the
/// original placement consumer is not reconstructed by this transport type.
/// No Default/Clone/Copy is supplied for unknown active application data.
#[derive(Event, Debug)]
pub struct InstaCreateStairPoint {
    observed: [u8; 0x70],
}
impl InstaCreateStairPoint {
    /// Accept a complete already observed payload, excluding its EventId.
    /// The writer always supplies kind0x3f, also checked by original placement.
    pub fn from_observed(observed: [u8; 0x70]) -> Self {
        assert_eq!(observed[0x68], 0x3f, "original stair creation kind");
        Self { observed }
    }
    pub fn observed(&self) -> &[u8; 0x70] {
        &self.observed
    }
}

/// DisplayChildAutoClutterInfo（O：0x60 B；`grow_one<DisplayChildAutoClutterInfo>`）。
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DisplayChildAutoClutterInfo {
    /// `+0x00..+0x1f`：`clutter_mesh_ty_data()` 的 4 个 qword。
    pub mesh: [u64; 4],
    /// `+0x20..+0x4f`：变换记录（0x30 stride）的 6 个 qword。
    pub transform: [u64; 6],
    /// `+0x50`：变换记录 `+0xc` 的 u32。
    pub field: u32,
}

/// 单个子项输入（I：宿主从 0xa0B 记录的 +0x50 子表、0x60 父表、0x30 变换表解析）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct AutoClutterChild {
    /// 父记录 `+0x00` 类型（O：`*puVar19 < 3` 门）。
    pub parent_tag: u32,
    /// 父记录 `+0x10`（O：HashMap 键 `DecoratorId`）。
    pub parent_key: u32,
    /// 子项信息（0x60 B）。
    pub info: DisplayChildAutoClutterInfo,
}

/// clutter id → 子项列表（I：宿主维护；对应 fresh 的 0xa0B 记录表 + 父/变换表）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AutoClutterChildTable(pub HashMap<u64, Vec<AutoClutterChild>>);

/// clutter→parent 映射（O：`HashMap<DecoratorId, Vec<DisplayChildAutoClutterInfo>>`；
/// `maintain_autoclutter_parent_hashmap` 在函数内维护）。
#[derive(Resource, Clone, Debug, Default)]
pub struct AutoClutterParentMap(pub HashMap<u32, Vec<DisplayChildAutoClutterInfo>>);

impl AutoClutterParentMap {
    /// 维护一条记录（已有键 push，新键新建 Vec）。
    pub fn maintain(&mut self, parent: u32, info: DisplayChildAutoClutterInfo) {
        self.0.entry(parent).or_default().push(info);
    }
    pub fn parent_of(&self, parent: u32) -> Option<&Vec<DisplayChildAutoClutterInfo>> {
        self.0.get(&parent)
    }
}

/// `ui_convert_autoclutter_to_decoclutter` @0x14099ca60。
pub fn ui_convert_autoclutter_to_decoclutter(
    mode: Res<AppMode>,
    mut right_clicks: EventReader<EvRightClickedAutoClutter>,
    children: Res<AutoClutterChildTable>,
    mut parents: ResMut<AutoClutterParentMap>,
    index: Res<super::AutoClutterEntityIndex>,
    mut hits: EventWriter<super::rmb_autoclutter::RmbAutoClutterHit>,
) {
    // **主循环**：点击 → 查 clutter 记录 → 逐子项维护 parent 映射。
    // Preserve the existing parent-map adapter, but forward only the newest
    // click. B @14099d6c8..d7ca consumes the complete reader and validates that
    // latest generation-bearing entity ID before invoking the RMB helper.
    let mut latest = None;
    for click in right_clicks.read() {
        latest = Some(click.entity);
        let Some(list) = children.0.get(&click.entity) else {
            continue;
        };
        for child in list {
            // O：`if (*puVar19 < 3)` 才进入转换；否则跳过。
            if child.parent_tag < 3 {
                parents.maintain(child.parent_key, child.info);
            }
        }
    }

    if let Some(entity) = latest.filter(|entity| index.0.contains_key(entity)) {
        hits.write(super::rmb_autoclutter::RmbAutoClutterHit(entity));
    }

    // **第二事件队列**（O：d2s 索引 + 位图命中 → 调 `handle_rmb_autoclutter`）。
    // crate 以产出 RmbAutoClutterHit 事件承载（宿主实体表命中判定在宿主侧，I）。

    // **尾部模式门**（O：`disc-6 < 0xf → disc-5`，否则 0；接受 disc==0 或
    // disc==9 ∧ payload 低字节==3）。
    let disc = mode.native_tag();
    let accepted = disc == 0 || matches!(*mode, AppMode::Eraser(EraserLayer::Bulldoze));
    if !accepted {
        return;
    }
    // 之后：`Rapier::raycast`（命中 tag != 2 ∧ 标签 0x0c）→ DecoratorStorage 探测 →
    // `UiState::add` → 0x60 记录类型 3..5 子跳表（&DAT_142abbcdc，UNRECOVERED_JUMPTABLE）。
    // 宿主射线/装饰存储/UI 名队列未接入，登记待接（不伪造实体/事件）。
}

#[cfg(test)]
mod lifecycle_integration_tests {
    use super::*;
    use crate::{
        resources::history::{EditType, History, auto_clutter_edit::AutoClutterEdit},
        systems::{
            backend::input::input_record_replay::InputReplayCommand,
            history::recording::{history_finish_recording_system, register_recording_events},
            mode_manager::ClutterMeshTy,
            ui_clutter::{
                AutoClutterEntityIndex, AutoClutterRecord,
                registration::register_ui_clutter_resources,
                rmb_autoclutter::handle_rmb_autoclutter,
            },
        },
    };

    fn record() -> AutoClutterRecord {
        AutoClutterRecord {
            clutter_id: 73,
            // Source destination observed in native writer fixture deco[0].
            // Its placement enum semantics are outside this arrival test.
            destination: [0x75dd10c75433c8db, 0xe038d51dca8bf380, 0xecd9fb9341459e0e],
            position: glam::Vec3::new(1.0, 2.0, 3.0),
            w: 1.0,
            x: 0.0,
            y: 0.0,
            z: 0.0,
            entry_angle: 0.0,
            roof_space: false,
            local_x: 1.0,
            local_y: 0.0,
            ty: ClutterMeshTy::Bench,
        }
    }
    fn world() -> World {
        let mut world = World::new();
        register_ui_clutter_resources(&mut world);
        register_recording_events(&mut world);
        world.insert_resource(AppMode::default());
        world.insert_resource(History::<()>::empty_glade());
        world
            .resource_mut::<AutoClutterEntityIndex>()
            .0
            .insert(42, record());
        world
    }

    #[test]
    fn actual_latest_right_click_emits_creation_and_commits_its_history() {
        let mut world = world();
        assert!(!world.resource::<History<()>>().is_recording());
        world.send_event(EvRightClickedAutoClutter { entity: 42 });
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_convert_autoclutter_to_decoclutter,
                handle_rmb_autoclutter::<()>,
                history_finish_recording_system::<()>,
            )
                .chain(),
        );
        schedule.run(&mut world);
        let history = world.resource::<History<()>>();
        assert!(!history.is_recording());
        assert_eq!(history.ledger().entries.len(), 1);
        assert!(
            world
                .resource::<crate::systems::ui_clutter::DeletedAutoClutter>()
                .is_deleted(73)
        );
        assert!(
            !world
                .resource::<crate::systems::ui_clutter::DeletedAutoClutter>()
                .is_deleted(42)
        );
        assert!(
            matches!(&history.ledger().entries[0].edits[..], [EditType::AutoClutter(edit)] if **edit == (AutoClutterEdit::Delete { id: 73 }))
        );
        let events = world.resource::<Events<InstaCreateDeco>>();
        let mut cursor = events.get_cursor();
        let output = cursor.read(events).next().unwrap();
        assert_eq!(output.destination, record().destination);
        assert!(output.extra.is_none());
        assert_eq!(output.position, record().position);
        assert_eq!(output.ty, record().ty);
        let sync = world.resource::<Events<InputReplayCommand>>();
        assert_eq!(
            sync.get_cursor().read(sync).cloned().collect::<Vec<_>>(),
            [InputReplayCommand::Sync { record: true }]
        );
        // Actual placement consumption remains an explicit upper-module gap.
    }

    #[test]
    fn latest_invalid_entity_suppresses_earlier_valid_click_and_consumes_reader() {
        let mut world = world();
        // A Direct record has no entity identity and cannot bypass the native
        // generation-bearing entity guard on the ui_convert input path.
        world
            .resource_mut::<super::super::rmb_autoclutter::RmbAutoClutterDirect>()
            .0 = Some(record());
        world.send_event(EvRightClickedAutoClutter { entity: 42 });
        world.send_event(EvRightClickedAutoClutter { entity: 999 });
        let mut schedule = Schedule::default();
        schedule.add_systems(
            (
                ui_convert_autoclutter_to_decoclutter,
                handle_rmb_autoclutter::<()>,
            )
                .chain(),
        );
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        assert!(!world.resource::<History<()>>().is_recording());
        schedule.run(&mut world);
        assert!(world.resource::<Events<InstaCreateDeco>>().is_empty());
        world.send_event(EvRightClickedAutoClutter { entity: 999 });
        world.send_event(EvRightClickedAutoClutter { entity: 42 });
        schedule.run(&mut world);
        assert_eq!(world.resource::<Events<InstaCreateDeco>>().len(), 1);
    }
}

#[cfg(test)]
mod original_name_lookup_tests {
    use super::clutter_mesh_type_from_name;

    #[test]
    fn original_fallthrough_arms_and_duplicate_stump_name_keep_their_types() {
        for (name, tag) in [("tree_log", 35), ("tree_stump", 36),
            ("clutter/cart_w_pumpkins", 40), ("clutter/crates_w_pumpkins", 41),
            ("clutter/stall_veggies", 47), ("clutter/crate_fish", 48),
            ("clutter/door_krans_autumn", 53), ("clutter/cart_w_pumpkins_regular", 54)] {
            assert_eq!(clutter_mesh_type_from_name(name) as u8, tag);
        }
        assert_ne!(clutter_mesh_type_from_name("tree_stump") as u8, 42);
    }

    #[test]
    fn unknown_or_normalized_names_keep_the_original_unreachable_failure() {
        for name in ["", "clutter/tree_stump", "TREE_STUMP", "tree_stump_v2",
            "clutter/window_flowerbeds", "caller_defined_mesh"] {
            assert!(std::panic::catch_unwind(|| clutter_mesh_type_from_name(name)).is_err(),
                "unknown original name unexpectedly parsed: {name}");
        }
    }
}
