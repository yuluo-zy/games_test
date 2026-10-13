//! country-core：auto_clutter_edit / replay_auto_clutter 恢复草稿（B 独有历史体系，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/clutter-extra-analysis.md（2026-10-13 reva worker
//! `clutter-extra-owner` + radare2 RNE oracle 精读）。
//! 原模块（B 独有，A 无对应）：
//!   - crates/country-core/src/resources/history/auto_clutter_edit.rs（源 L8 impl$0）
//!   - crates/country-core/src/systems/history/replay_auto_clutter.rs（源 L16，closure$0 L38）
//!   - crates/country-core/src/systems/ui_clutter/defered_autoclutter_history.rs（L16–21）
//!   - crates/country-core/src/systems/ui_clutter/rmb_autoclutter.rs（L23–44）
//!   - crates/country-core/src/systems/ui_clutter/ui_autoclutter.rs（ui_convert…decoclutter）
//!
//! 语义：用户右键删除自动杂物 → ClutterMeshTy + 编辑记录 + AutoClutterEdit 历史命令
//! （serialize/deserialize 为 {"Delete":{"id":<u64>}}）→ 回放时按帧推进生长、完成即移除并
//! 清除该墙/装饰物的保护标记。
//!
//! VA（B）：AutoClutterEdit serialize 0x14003d300/0x140a18320（size 473）、deserialize
//! 0x140a18500（size 1682）；history_record_auto_clutter 0x1409a9160（size 346）；
//! handle_rmb_autoclutter 0x140976ca0（size 1359）；ui_convert_autoclutter_to_decoclutter
//! 0x14099ca60；replay_auto_clutter 0x140aa4a60。
//!
//! 依赖复用：bevy_ecs、serde/serde_json、hashbrown、alloc 直接复用。字段为证据；未确认
//! 保留 unknown。

use bevy_ecs::prelude::*;
use glam::Vec3;
use hashbrown::{HashMap, HashSet};
use std::sync::Arc;

/// 墙标识（country_core::resources::walls::WallId）。
pub type WallId = u64;

/// 装饰物标识对（replay 擦除 [DecoratorId, DecoratorId]）。
pub type DecoratorId = u64;

// ---- AutoClutterEdit（resources/history/auto_clutter_edit.rs）----

/// 自动杂物编辑（enum；PDB：enum2$，存于 Arc<AutoClutterEdit>）。
/// serde 证据：serialize 0x14003d300 写 `{"Delete":{"id":<u64>}}` —— 外层键
/// "Delete"（0x14292c1b7 len 6）、内层 serialize_entry<str,u64> 键 "id"（0x14292c1bd len 2）；
/// deserialize 0x140a18500 variant_seed+struct_variant，期望单键 map
/// （错误串 "map with a single key" @0x142ac7e80）。
/// 采用默认外部标签（无 serde 属性）：`{"Delete":{"id":...}}` 与证据一致
/// （内部标签 `#[serde(tag="Delete")]` 会产出 `{"Delete":"Delete",...}`，与反汇编不符，已修）。
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum AutoClutterEdit {
    /// 字段名 "id"（u64）。
    Delete { id: u64 },
}

impl AutoClutterEdit {
    /// 历史命令构造：history_record 写 `(1, 1, WallId)`（grow=1、count=1、id=WallId）。
    pub fn delete(wall: WallId) -> Arc<AutoClutterEdit> {
        Arc::new(AutoClutterEdit::Delete { id: wall })
    }
}

// ---- defered_autoclutter_history（systems/ui_clutter/defered_autoclutter_history.rs）----

/// 自动杂物编辑历史记录：遍历被编辑墙集合，逐墙写 AutoClutterEdit 历史命令。
/// 证据（history_record_auto_clutter，0x1409a9160，源 L16–21）：
/// - 遍历 HashSet<WallId>（stride 0x10 双段）；每墙 alloc (1,1,WallId) → History::add
pub fn history_record_auto_clutter(
    _edited_walls: &HashSet<WallId>,
    _history: &mut History,
) {
    for &wall in _edited_walls {
        _history.add(AutoClutterEdit::delete(wall)); // (grow=1, count=1, id=WallId)
    }
}

/// 历史（country_core::resources::history::History）。
pub struct History;
impl History {
    /// 证据：history_record / rmb 都先 start_input_recording 再 add。
    pub fn start_input_recording(&mut self) {}
    pub fn add(&mut self, _cmd: Arc<AutoClutterEdit>) {}
}

// ---- rmb_autoclutter（systems/ui_clutter/rmb_autoclutter.rs）----

/// 右键删除自动杂物。
/// 证据（handle_rmb_autoclutter，0x140976ca0，源 L23–44）：
/// - 命中墙：atan2f 命中点→朝向角；HashMap<WallId,()>::insert（记录被编辑墙）；
///   ClutterMeshTy::from() 定网格种类；写编辑记录（stride 0x48，进
///   indexmap Bucket<(SubworldColliderHandle,SubworldColliderHandle),MeshIntersectionArtifacts>）：
///   +0 WallId、+0x10 网格种类、+0x19 kind byte=3、+0x2c 朝向角、+0x30 网格类型、+0x40 等；
///   History::start_input_recording() + History::add((1,1,WallId))
/// - 未命中：碰撞射线查询（Rapier raycast）找墙链命中并算角度
pub fn handle_rmb_autoclutter(
    _ray_origin: Vec3,
    _ray_dir: Vec3,
    _picked: &PickedWall,
    _history: &mut History,
    _edited: &mut HashSet<WallId>,
) {
    if _picked.is_hit {
        _edited.insert(_picked.wall);
        let _mesh_ty = ClutterMeshTy::from(_picked); // 定网格种类
        _history.start_input_recording();
        _history.add(AutoClutterEdit::delete(_picked.wall));
        unknown(); // 编辑记录（stride 0x48）写入 indexmap
    } else {
        unknown(); // Rapier raycast 拾取链
    }
}

/// 拾取命中（in_RDX）。
pub struct PickedWall {
    pub is_hit: bool,
    pub wall: WallId,
}

/// 杂物网格种类（country_core::resources::clutter_meshes::ClutterMeshTy）。
pub struct ClutterMeshTy;
impl ClutterMeshTy {
    pub fn from(_pick: &PickedWall) -> ClutterMeshTy {
        ClutterMeshTy
    }
}

// ---- ui_autoclutter（systems/ui_clutter/ui_autoclutter.rs）----

/// UI 转装饰物：Rapier::raycast 拾取 → ClutterMeshTy::from / clutter_mesh_ty_data →
/// 转 decoclutter；复用 handle_rmb_autoclutter。
pub fn ui_convert_autoclutter_to_decoclutter(_ray: Vec3) {
    unknown();
}

// ---- replay_auto_clutter（systems/history/replay_auto_clutter.rs）----

/// 自动杂物编辑回放（源 L16–38；closure$0 L38）。
/// 证据（replay_auto_clutter，0x140aa4a60）：
/// - state+0x28 = 每帧生长 delta（fVar3）；state[3]==0 早退
/// - 遍历历史命令 Vec（stride 0x30），`*cmd==9`（'\t'，AutoClutterEdit 变体 tag）才处理：
///   - +0x18 生长值==0（未开始）→ 抹保护：HashMap<WallId,()>::insert（byte+0x1c==0）或
///     hash 集遍历 RawTable::erase<[DecoratorId,DecoratorId]>
///   - +0x8 bit0==0 → 收集索引；否则 fVar15=clamp(cmd+0xc / delta, min 0.05)，
///     fVar15 < cmd+0x18 → 收集（生长完成）；否则 cmd+0x18 += delta
/// - 反转索引列表逐条 Vec::remove（memmove）+ 按 kind byte drop：
///   0/1/2 Arc<RasterEdit>、3 Arc<TerrainEdit>、4 WallReplayCmd、5 Arc<RoofEdit>、
///   6/7 Arc<DecoratorHistoryEdit>、8 Arc<TreeEdit>、9 Arc<Mutex<CrashMiscInfo>>、default 其他
pub fn replay_auto_clutter(
    _delta: f32,
    _commands: &mut Vec<ReplayCommand>,
    _edited_protected: &mut EditedClutterProtection,
) {
    let mut pending = Vec::new();
    for (i, cmd) in _commands.iter_mut().enumerate() {
        if cmd.tag != 9 {
            continue;
        }
        if cmd.grow == 0.0 {
            // 抹保护（cVar6==0 insert 空，否则 erase DecoratorId 对）。
            if cmd.flag == 0 {
                _edited_protected.walls.insert(cmd.wall, ());
            } else {
                _edited_protected.erase_decorator_pair(cmd.wall);
            }
        }
        if cmd.marked {
            pending.push(i);
        } else {
            let f = (cmd.count / _delta).max(0.05);
            if f < cmd.grow {
                pending.push(i);
            } else {
                cmd.grow += _delta; // lVar24+0x28
            }
        }
    }
    for &i in pending.iter().rev() {
        _commands.remove(i); // memmove + drop（kind byte 分派）
    }
}

/// 回放命令（stride 0x30；tag 9 = AutoClutterEdit 变体）。
pub struct ReplayCommand {
    /// +0x00 tag（'\t'=9）。
    pub tag: u8,
    /// +0x08 bit0（标记移除）。
    pub marked: bool,
    /// +0x0c f32（count/缩放；`f=cmd+0xc/delta`）。
    pub count: f32,
    /// +0x10 WallId。
    pub wall: WallId,
    /// +0x18 f32 生长值（+= delta）。
    pub grow: f32,
    /// +0x1c byte（保护分支 cVar6）。
    pub flag: u8,
    /// +0x20/+0x28 unknown。
    pub _rest: [u8; 16],
}

/// 被编辑自动杂物的保护集。
pub struct EditedClutterProtection {
    /// HashMap<WallId,()>（replay 抹除用）。
    pub walls: HashMap<WallId, ()>,
}

impl EditedClutterProtection {
    pub fn erase_decorator_pair(&mut self, _wall: WallId) {
        // 证据：RawTable::erase<([DecoratorId,DecoratorId],())>。
        unknown();
    }
}

fn unknown() -> ! {
    unimplemented!("反汇编级证据：精确逻辑见 evidence/multi-baseline/clutter-extra-analysis.md")
}
