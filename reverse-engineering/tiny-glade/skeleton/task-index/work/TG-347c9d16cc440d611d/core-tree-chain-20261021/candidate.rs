//! 树木放置 UI 与编辑命令；保留当前系统和权威 TreeStorage。
//!
//! B 原字节 142269970：以存储索引初始化本地 fastrand1.8.0，依次采样
//! 缩放、变体、旋转，提交 Insert 后录入历史。14226a170 的三个 ICF
//! 同址函数由 ActiveSession 的 PDB inline 确认为 olden/winter/autumn。
//! UI seed_cursor_icon 的原闭包仍待恢复，不能把 raw 工厂视图视为已渲染。

use bevy_ecs::prelude::*;
use glam::Vec2;

use country_core::resources::cursor_terrain_raycast::CursorTerrainRaycast;
use country_core::resources::glade_border::{
    DisplayOutOfBorder, FeedbackInputOutOfBorder, GladeBorder,
};
use country_core::resources::history::tree_edit::InsertTreeCmd;
use country_core::resources::history::{EditType, History};
use country_core::resources::session::ActiveSession;
use country_core::resources::tree_storage::TreeStorage;
use country_core::systems::collision::world::{RAYCAST_WORLD_RESULT_MISS, RaycastWorld};
use country_core::systems::mode_manager::{AppMode, EditSubject};
use country_core::systems::ui::resources::ui_state::UiState;
use country_core::systems::ui::resources::widget_params::{
    UiSounds, WidgetParams, WidgetVolumeShape,
};
use country_core::systems::ui::signifiers::SignifierFactory;
use country_core::systems::ui::signifiers::signifier_create::UiSignifierStream;
use country_core::systems::ui_edit_focus::{GeoSource, geo_src_to_edit_subject};

use crate::PlaceTreeCmd;

/// 进程全局 `_ZN12country_core26_DEV_OVERRIDE_IS_DEMO_MODE...E` @0x14362c098
/// （O：ui_place_tree 0x142269170 行 51-52、ui_delete_tree 0x141bd2510 行 61
/// `!=0 → return`）。非 0 时禁用户工具。本 crate 侧建模为编译期常量；
/// 真实值由宿主进程写入（非 0 早退），接线时由宿主覆盖。
pub const DEV_OVERRIDE_IS_DEMO_MODE: bool = false;

/// 水下重映射常量（O：ui_place_tree 0x142269170 行 152-154：`y<0 →
/// (y/-2.5)*-1.2 + (1-y/-2.5)*0.0`）。字节直读 `DIVSS XMM0,-2.5` /
/// `MULSS XMM0,-1.2`。
pub const WATER_REMAP_DIVISOR: f32 = -2.5;
pub const WATER_REMAP_TARGET: f32 = -1.2;

/// B 142269c00..c8c：缩放在 .5 与 .9 之间插值。
pub const TREE_SCALE_MIN: f32 = 0.5;
pub const TREE_SCALE_MAX: f32 = 0.9;
/// B 142269aa0 用 MOVSS 载入 .5，并把高位清零；旋转 lane 的下限为零。
pub const TREE_ROTATION_MAX: f32 = 360.0;
pub const TREE_ROTATION_MIN: f32 = 0.0;

/// 树 prefab 路径表（O：0x1430351d0 区字符串直读，2026-10-17）：
/// branchy2(22) / branchy1(22) / autumn/branchy2(29) / autumn/branchy1(29) /
/// naked1(26) / naked2(26) / naked1_no_mushrooms(39)。
pub const TREE_PREFAB_BRANCHY: [&str; 2] = ["clearing/tree/branchy2", "clearing/tree/branchy1"];
pub const TREE_PREFAB_AUTUMN: [&str; 2] = [
    "clearing/tree/autumn/branchy2",
    "clearing/tree/autumn/branchy1",
];
pub const TREE_PREFAB_NAKED: [&str; 2] =
    ["clearing/naked_tree/naked1", "clearing/naked_tree/naked2"];
pub const TREE_PREFAB_NO_MUSHROOMS: &str = "clearing/naked_tree/naked1_no_mushrooms";

/// B 14226a170：仅按存储索引奇偶及会话主题选择七种原 prefab。
/// 参数次序保留现有调用面；ICF 别名不能解释为加载、损坏或拖动状态。
pub fn random_tree_prefab(
    selector: u8,
    is_olden: bool,
    is_halloween: bool,
    is_winter: bool,
    is_autumn: bool,
) -> &'static str {
    let bit = (selector & 1) as usize;
    if is_winter && bit == 0 {
        return TREE_PREFAB_NO_MUSHROOMS;
    }
    if (is_olden || is_halloween) && bit == 0 {
        return TREE_PREFAB_NAKED[0];
    }
    let table = if is_autumn {
        &TREE_PREFAB_AUTUMN
    } else {
        &TREE_PREFAB_BRANCHY
    };
    table[bit]
}

/// 同一存储索引的放置属性使用同一随机流，不读取线程全局 RNG。
/// 原版整数采样为 i32(0..10)，由 pinned fastrand 保留拒绝重采样。
fn placement_randoms(index: u64) -> (f32, i32, f32) {
    let rng = fastrand::Rng::with_seed(index);
    let scale = rng.f32();
    let variant = rng.i32(0..10);
    let rotation = rng.f32();
    (scale, variant, rotation)
}

/// 水下 y 重映射（O：ui_place_tree 内联 `remap_pos_y_if_in_water`）：
/// `y' = (y / -2.5) * -1.2 + (1.0 - y / -2.5) * 0.0`。
pub fn remap_pos_y_if_in_water(y: f32) -> f32 {
    if y < 0.0 {
        let a = y / WATER_REMAP_DIVISOR;
        a * WATER_REMAP_TARGET + (1.0 - a) * 0.0
    } else {
        y
    }
}

/// legacy_raycast miss 判断（O 调用点 0x1422692ba：`cmp dword [rbp+0xf8],2`，
/// 即结果 **+0x30 槽位型** == 2）。
///
/// native `RaycastWorld::legacy_raycast` 在 miss 时把 u32 2 的位型写入 +0x30，
/// 故 `hit.toi.to_bits() == 2` 逐位等价。非 native-physics 壳返回
/// `RaycastWorldResult::miss()`（toi 未写位型、判别 +0x34 == 2），此分支作
/// shell seam 一并视为 miss（登记 I；native 下两条件恒等价）。
fn legacy_raycast_miss(hit: &country_core::systems::collision::world::RaycastWorldResult) -> bool {
    hit.toi.to_bits() == RAYCAST_WORLD_RESULT_MISS || hit.is_miss()
}

/// `ui_place_tree` @0x142269170（O 面）。
#[allow(clippy::too_many_arguments)]
pub fn ui_place_tree(
    mode: Res<AppMode>,
    ui: Res<UiState>,
    mut signifiers: ResMut<UiSignifierStream>,
    cursor: Res<CursorTerrainRaycast>,
    mut place_cmds: EventWriter<PlaceTreeCmd>,
    raycast_world: Res<RaycastWorld>,
    camera_ray: Res<country_core::resources::mouse_terrain_raycast::CameraCursorRay>,
    cursor_ss: Res<country_core::resources::mouse_terrain_raycast::CursorPositionSS>,
    window: Res<country_core::resources::window::WindowSize>,
    config: Res<country_core::game_config::GameConfig>,
    mut border: (
        Res<GladeBorder>,
        EventWriter<DisplayOutOfBorder>,
        EventWriter<FeedbackInputOutOfBorder>,
    ),
) {
    // 门禁（O：`*AppMode==0x10 && DEMO==0`）。
    if DEV_OVERRIDE_IS_DEMO_MODE || mode.native_tag() != AppMode::Tree.native_tag() {
        return;
    }
    // O：widget（WidgetParams::new → occlude_by_world(false) → sounds(empty) → add）。
    // volume 字节：tag@0 = 5、f32@4 = f32::MAX（size.y 原生未写，U）。
    let volume = WidgetVolumeShape {
        tag: 5,
        layer: 0,
        size: Vec2::new(f32::MAX, 0.0),
        ..Default::default()
    };
    let ctx = ui.add(
        "tree_canvas",
        WidgetParams::new(volume, [0; 2])
            .occlude_by_world(false)
            .sounds(UiSounds::empty()),
    );
    // O：legacy_raycast（0x1408cc680）+ geo_src_to_edit_subject（0x140977f50）；
    // 命中且 subject != None → 早退。
    let hit = raycast_world.legacy_raycast(camera_ray.origin, camera_ray.dir);
    if !legacy_raycast_miss(&hit) {
        let subject = geo_src_to_edit_subject(GeoSource::from_hit_handle(hit.handle));
        if !matches!(subject, EditSubject::None) {
            return;
        }
    }
    let position = cursor.position();
    let center = Vec2::new(position.x, position.z);
    if !border.0.is_pos_inside(center) {
        // O：界外 → DisplayOutOfBorder；clicked 时再 FeedbackInputOutOfBorder。
        border.1.write(DisplayOutOfBorder);
        if !ctx.clicked() {
            return;
        }
        border.2.write(FeedbackInputOutOfBorder);
        return;
    }
    // O：界内且 is_active → 指示符。
    // label = format!("decorator_{}", CursorPositionSS)；记录 20 B
    // {1,1,cursor_ss 位型, window.as_vec2().y, config.ui.icon_size_mult}；
    // SignifierFactory::new(label, record, 0x143035160) + write（逐 5 qword）。
    if ctx.is_active() {
        let ss = cursor_ss.0;
        let label = format!("decorator_{ss}");
        let mut record = [0u8; 0x20];
        record[0..8].copy_from_slice(&1u64.to_le_bytes());
        record[8..16].copy_from_slice(&1u64.to_le_bytes());
        record[16..20].copy_from_slice(&ss.x.to_le_bytes());
        record[20..24].copy_from_slice(&ss.y.to_le_bytes());
        record[24..28].copy_from_slice(&window.as_vec2().y.to_le_bytes());
        record[28..32].copy_from_slice(&config.ui.icon_size_mult.to_le_bytes());
        // 原生把 20 B 记录的所有权交给 signifier 记录（`inner`）；core 记录只存
        // 裸指针，故以 leak 保持载荷存活（所有权/唤醒登记 U）。
        let inner = Box::leak(Box::new(record)) as *const u8;
        let factory = SignifierFactory::new(label.as_bytes(), inner, 0x1430_3516_0);
        signifiers.write_factory(&factory);
    }
    if !ctx.clicked() {
        return;
    }
    // O：clicked → position()（第二次采样）→ 水下重映射 → PlaceTreeCmd。
    let mut pos = cursor.position();
    if pos.y < 0.0 {
        pos.y = remap_pos_y_if_in_water(pos.y);
    }
    place_cmds.write(PlaceTreeCmd { position: pos });
}

/// B 142269970：从真实位置事件生成完整插入命令，并录入同一份历史数据。
/// Insert 先入队；缺少历史录制时保留错误，不能静默丢失撤销/保存记录。
pub fn place_tree<S: Send + Sync + 'static>(
    mut storage: ResMut<TreeStorage>,
    mut commands: EventReader<PlaceTreeCmd>,
    mut inserts: EventWriter<InsertTreeCmd>,
    mut history: ResMut<History<S>>,
    session: Res<ActiveSession>,
) {
    use country_core::resources::history::tree_edit::{TreeEdit, TreeEditData};
    use std::sync::Arc;

    for cmd in commands.read() {
        let index = storage.next_index();
        let (r1, variant, r2) = placement_randoms(index.0);
        let prefab = random_tree_prefab(
            index.0 as u8,
            session.is_olden(),
            session.meta.is_halloween_any(),
            session.is_winter(),
            session.is_autumn(),
        );
        let scale = r1 * TREE_SCALE_MAX + (1.0 - r1) * TREE_SCALE_MIN;
        let rotation = r2 * TREE_ROTATION_MAX + (1.0 - r2) * TREE_ROTATION_MIN;
        // B 142269cc7 写入 1；该字节原样复制到 Insert 与 TreeEdit 的布尔字段。
        let data = TreeEditData {
            storage_index: Some(index),
            id: variant,
            position: cmd.position,
            previous_position: glam::Vec3::ZERO,
            scale,
            rotation_degrees: rotation,
            prefab_name: prefab.to_string(),
            is_preplaced: true,
        };
        inserts.write(InsertTreeCmd {
            id: index.0,
            rotation_degrees: rotation,
            variant,
            is_preplaced: true,
            position: cmd.position,
            scale,
            prefab_name: prefab.to_string(),
        });
        history
            .add(EditType::Tree(Arc::new(TreeEdit::Place(data))))
            .expect("tree placement requires an active history recording");
    }
}

/// `seed_cursor_icon`（行 115，内联；O 名——光标图标种子逻辑并入调用点，
/// 本 crate 无独立业务面）。
pub fn seed_cursor_icon() {}

#[cfg(test)]
mod placement_producer_tests {
    use super::*;
    use country_core::resources::{
        history::tree_edit::TreeEdit,
        session::{GladeType, SessionId, SessionMeta, SessionMutator},
    };

    fn run(glade: GladeType, mutators: Vec<SessionMutator>, global_seed: u64) -> (InsertTreeCmd, World) {
        fastrand::seed(global_seed);
        let mut world = World::new();
        world.init_resource::<TreeStorage>();
        world.init_resource::<Events<PlaceTreeCmd>>();
        world.init_resource::<Events<InsertTreeCmd>>();
        let mut history = History::<()>::empty_glade();
        history.start_input_recording(16).unwrap();
        world.insert_resource(history);
        world.insert_resource(ActiveSession::new(
            SessionId::new(),
            SessionMeta { glade, mutators, image: None },
        ));
        world.send_event(PlaceTreeCmd { position: glam::Vec3::new(1.0, -1.2, 3.0) });
        let mut schedule = Schedule::default();
        schedule.add_systems(place_tree::<()>);
        schedule.run(&mut world);
        let events = world.resource::<Events<InsertTreeCmd>>();
        let mut reader = events.get_cursor();
        let insert = reader.read(events).next().unwrap().clone();
        (insert, world)
    }

    #[test]
    fn frozen_native_placement_fixtures_keep_rng_draw_order_and_zero_rotation_lane() {
        for (index, variant, scale_bits, rotation_bits) in [
            (0u64, 9, 0x3f39570e, 0x43497cff),
            (1u64, 1, 0x3f5a6391, 0x4283f7af),
            (2u64, 3, 0x3f3341fb, 0x42dd76c0),
            (7u64, 9, 0x3f5a8409, 0x431f0296),
            (42u64, 2, 0x3f58aa48, 0x42751c53),
            (100000u64, 6, 0x3f01b40a, 0x43520800),
            (1311768467463790320u64, 6, 0x3f575867, 0x42e7e67e),
            (18446744073709551615u64, 3, 0x3f5cdef1, 0x431d14a7),
        ] {
            let (r1, actual_variant, r2) = placement_randoms(index);
            let scale = r1 * TREE_SCALE_MAX + (1.0 - r1) * TREE_SCALE_MIN;
            let rotation = r2 * TREE_ROTATION_MAX + (1.0 - r2) * TREE_ROTATION_MIN;
            assert_eq!(actual_variant, variant, "index={index}");
            assert_eq!(scale.to_bits(), scale_bits, "index={index}");
            assert_eq!(rotation.to_bits(), rotation_bits, "index={index}");
        }
    }

    #[test]
    fn placement_uses_session_theme_and_same_full_data_for_insert_and_history() {
        for (glade, mutators, expected) in [
            (GladeType::Summer, vec![], TREE_PREFAB_BRANCHY[0]),
            (GladeType::Olden, vec![], TREE_PREFAB_NAKED[0]),
            (GladeType::Flowery, vec![], TREE_PREFAB_BRANCHY[0]),
            (GladeType::Autumn, vec![], TREE_PREFAB_AUTUMN[0]),
            (GladeType::Winter, vec![], TREE_PREFAB_NO_MUSHROOMS),
            (GladeType::Summer, vec![SessionMutator::Halloween2024], TREE_PREFAB_NAKED[0]),
        ] {
            let (insert, world) = run(glade, mutators, 1);
            assert_eq!(insert.id, 0);
            assert_eq!(insert.prefab_name, expected);
            assert!(insert.is_preplaced);
            let pending = world.resource::<History<()>>().pending().unwrap();
            let EditType::Tree(edit) = &pending.edits[0] else { panic!("expected tree edit") };
            let TreeEdit::Place(data) = edit.as_ref() else { panic!("expected place edit") };
            assert_eq!(data.storage_index.unwrap().0, insert.id);
            assert_eq!(data.id, insert.variant);
            assert_eq!(data.position, insert.position);
            assert_eq!(data.scale.to_bits(), insert.scale.to_bits());
            assert_eq!(data.rotation_degrees.to_bits(), insert.rotation_degrees.to_bits());
            assert_eq!(data.prefab_name, insert.prefab_name);
            assert_eq!(data.is_preplaced, insert.is_preplaced);
        }
    }

    #[test]
    fn unrelated_global_rng_seed_does_not_change_the_same_tree() {
        let (a, _) = run(GladeType::Autumn, vec![], 1);
        let (b, _) = run(GladeType::Autumn, vec![], u64::MAX);
        assert_eq!(a.variant, b.variant);
        assert_eq!(a.scale.to_bits(), b.scale.to_bits());
        assert_eq!(a.rotation_degrees.to_bits(), b.rotation_degrees.to_bits());
        assert_eq!(a.prefab_name, b.prefab_name);
    }

    #[test]
    fn missing_history_recording_fails_after_the_insert_was_submitted() {
        let mut world = World::new();
        world.init_resource::<TreeStorage>();
        world.init_resource::<Events<PlaceTreeCmd>>();
        world.init_resource::<Events<InsertTreeCmd>>();
        world.insert_resource(History::<()>::empty_glade());
        world.insert_resource(ActiveSession::new(SessionId::new(), SessionMeta::default()));
        world.send_event(PlaceTreeCmd { position: glam::Vec3::ONE });
        let mut schedule = Schedule::default();
        schedule.add_systems(place_tree::<()>);
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| schedule.run(&mut world)));
        assert!(result.is_err());
        let events = world.resource::<Events<InsertTreeCmd>>();
        let mut cursor = events.get_cursor();
        assert_eq!(cursor.read(events).count(), 1);
        assert!(world.resource::<History<()>>().pending().is_none());
    }

}
