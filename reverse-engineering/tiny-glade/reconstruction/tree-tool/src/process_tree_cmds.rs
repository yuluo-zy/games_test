//! tree-tool：process_tree_cmds 业务恢复草稿（B 主基线，反汇编级证据更新）
//!
//! 依据：evidence/multi-baseline/tree-process-cmds-analysis.md（2026-10-13 reva/radare2
//! 精读 0x142267430 更新）。字段布局现为证据：
//! - 事件槽 stride 与槽内偏移来自 reva（B）与 radare2（RNE oracle）逐条一致的反汇编。
//! - 索引字段以完整 8 字节读取并作 TreeStorage hash 键（u64）；`[infer]` 已移除或降级为
//!   `unknown`。未确认字段保留 `unknown`，不伪造。
//!
//! 依赖复用：bevy_ecs 0.16、glam 0.29.3 直接复用，不还原其实现。bevy EventInstance 槽
//! 含事件书签（sound 事件见 count/id 写在槽 @+0，spawn 事件见 @+0x48；依赖内部放置不统一）。
//! 本文件命令结构 = 游戏 payload（字段偏移为槽内偏移减去 bevy 书签后的 payload 偏移）。

use bevy_ecs::change_detection::{Res, ResMut};
use bevy_ecs::event::{EventReader, EventWriter};
use bevy_ecs::prelude::*;
use bevy_ecs::query::Query;
use bevy_ecs::system::Commands;
use glam::Vec3;
use std::collections::HashMap;

// ---- country_core::resources::tree_storage ----

/// 树存储：index(u64) → {prefab_index u32, entity u32} 的 hashbrown 映射。
/// 布局证据（0x140a38460/0x140a38560/0x140a385a0，reva 反汇编）：
/// - +0x18：u64 计数/非空标记（get 先判 0→None）
/// - +0x20：hash map（hashbrown）
/// - +0x30：next_index u64；`insert(index)` 执行 `next_index = max(next_index, index+1)`
#[derive(Default)]
pub struct TreeStorage {
    /// 树 index → 记录值（hashbrown，键 u64；无独立 entries Vec）。
    map: HashMap<u64, TreeEntryValue>,
    /// 单调分配的下一个索引（u64）。
    next_index: u64,
}

/// 单棵已放置树的记录值（hash 值，8 字节）。
/// 证据（Move 分支 *entry 读取）：低 32 = prefab 查询表下标；高 32 = 预制体实体号。
pub struct TreeEntryValue {
    /// prefab 查询表下标（低 32 位）。
    pub prefab_index: u32,
    /// 预制体实体号（高 32 位）。
    pub entity: u32,
}

impl TreeStorage {
    pub fn next_index(&self) -> u64 {
        self.next_index
    }
    /// 登记索引并推进 `next_index = max(next_index, index+1)`。
    /// 返回占用状态（u64；调用方在 insert 分支检测其高 32 位非零以决定是否
    /// DespawnPrefabCmd——槽位已占用的替换语义，部分确认）。
    pub fn insert(&mut self, index: u64) -> u64 {
        let inserted = self.map.insert(index, TreeEntryValue { prefab_index: 0, entity: 0 });
        self.next_index = self.next_index.max(index + 1);
        u64::from(inserted.is_some())
    }
    pub fn get(&self, index: u64) -> Option<&TreeEntryValue> {
        self.map.get(&index)
    }
    pub fn remove(&mut self, index: u64) -> Option<TreeEntryValue> {
        self.map.remove(&index)
    }
}

// ---- 命令事件（bevy EventInstance 槽 = {u64 id @+0, payload}；stride = 槽总大小）----

/// 移动树的命令（country_core::resources::tree_storage::MoveTreeCmd）。
/// 事件槽 0x20；payload 20B：index u64 @+0，position Vec3 @+8。
pub struct MoveTreeCmd {
    /// 目标树索引（8 字节读取，TreeStorage hash 键）。
    pub index: u64,
    /// 新位置。
    pub position: Vec3,
}

/// 删除树的历史命令（history::tree_edit，serde 序列化）。
/// 事件槽 0x10；payload 8B：index u64 @+0。
pub struct DeleteTreeCmd {
    /// 树索引（8 字节读取）。
    pub index: u64,
}

/// 插入树的历史命令（history::tree_edit，serde 序列化）。
/// 事件槽 0x48；payload 64B，字段如下。
pub struct InsertTreeCmd {
    /// 树 prefab 名（String：ptr @+0, len @+8）。
    pub name: String,
    /// 位置（x,y @+0x10, z @+0x18）。
    pub position: Vec3,
    /// 高度类标量 f32 @+0x1c（inverse_lerp 输入）。
    pub height: f32,
    /// f32 @+0x20（unknown 语义）。
    pub field_20: f32,
    /// flags u32 @+0x24（低 2 位参与拖拽门控）。
    pub flags: u32,
    /// byte @+0x28（spawn 门控输入；unknown 语义）。
    pub flag_byte: u8,
    /// 树索引 u64 @+0x30（TreeStorage::insert 参数）。
    pub index: u64,
    /// @+0x38..0x40 未读字段（unknown）。
    pub unknown_tail: [u8; 8],
}

// ---- country_core::systems::prefab ----

/// 预制体实例数据组件。
/// 证据：position Vec3 @+0（12B）；f32 @+0xc（inverse_lerp 读取，unknown 语义）。
pub struct PrefabInstanceData {
    /// 位置（Move 分支经 Query fetch 写入组件 +0）。
    pub position: Vec3,
    /// f32 @+0xc（unknown，疑似水面/高度缓存）。
    pub field_c: f32,
}

/// 生成预制体命令。
/// 事件槽 0x50。payload 起始于槽 @+0（与 Move/Delete/Insert 不同：该槽的 bevy 书签 count/id
/// 写在 @+0x48 尾部，见反汇编 0x142267e43 保存 [Events+0x40]→@+0x48）。payload 72B 字段如下，
/// 其余 unknown。
pub struct SpawnPrefabCmd {
    /// prefab 名（ptr @+0, len @+8）。
    pub name: String,
    /// u64 @+0x10（unknown）。
    pub field_10: u64,
    /// 位置（x,y @+0x18, z @+0x20）。
    pub position: Vec3,
    /// f32 @+0x24（unknown）。
    pub field_24: f32,
    /// f32 @+0x28（unknown）。
    pub field_28: f32,
    /// flags u32 @+0x2c。
    pub flags: u32,
    /// f32 @+0x30（=1.0，unknown）。
    pub field_30: f32,
    /// 其余 @+0x34..0x48（unknown）。
    pub unknown_tail: [u8; 0x14],
}

/// 销毁预制体命令。
/// 事件槽 0x10；payload 8B：u64 @+0。
pub struct DespawnPrefabCmd {
    /// 值（unknown：疑似树索引/实体；Insert 分支由 TreeStorage::insert 返回值写入）。
    pub value: u64,
}

// ---- country_core::systems::audio::tree_audio ----

/// 树音效事件。
/// 事件槽 0x20；payload 20B：kind u8 @+0，in_water bool @+1，flag bool @+2，
/// height f32 @+4，position Vec3 @+8。实测 kind：insert=0 / move=1 / delete=2。
pub struct PlayTreeSoundCmd {
    /// 音效种类（证据：三处事件写 kind 字节 0/1/2）。
    pub kind: u8,
    /// 是否在水中（bilinear_sample_at_world_xz > 0.0）。
    pub in_water: bool,
    /// 附加标志（Insert 分支 = 名字匹配 & (deny_load | is_corrupted)）。
    pub flag: bool,
    /// 高度类标量（inverse_lerp 结果）。
    pub height: f32,
    /// 位置。
    pub position: Vec3,
}

// ---- 会话/水域（依赖资源，字段仅证据级，内部不还原）----

/// 活动会话（country_core::resources::session::ActiveSession）。
/// 证据：内容 +0x58 为会话状态对象，Insert 分支调用其 is_dragging / deny_load /
/// is_corrupted 方法门控种树。对象实际 Rust 类型 unknown。
pub struct ActiveSession {
    /// [evidence] +0x58 处会话状态对象（内部布局 unknown）。
    pub state: SessionState,
}

/// ActiveSession[+0x58] 处对象（仅确认三个查询方法行为，内部 unknown）。
pub struct SessionState {
    pub _private: [u8; 0],
}

/// 水面栅格（country_core::resources::world_raster_defs::WaterRaster）。
/// 证据：内容双缓冲 RasterBuffer——指针对 @+0x8/+0x10、@+0x20/+0x28，字节标志 @+0x298
/// 选当前读缓冲；`bilinear_sample_at_world_xz<RasterBufferRef<512>>` 采样 > 0.0 判定入水。
/// 注意：+0x298 属于 WaterRaster，不属于 TreeStorage。
pub struct WaterRaster {
    pub _private: [u8; 0],
}

/// 消费树编辑命令，同步 TreeStorage 与预制体实例。
///
/// 签名来自 PDB 调用者泛型（硬证据）。三段事件处理顺序为 **Move → Insert → Delete**
/// （源码行 43–74 → 74–176 → 182–201；反汇编分支头 0x142267640 / 0x142267a29 /
/// 0x142268201，相对入口偏移 0x210 / 0x5F9 / 0xDD1）。
pub fn process_tree_cmds(
    mut tree_storage: ResMut<TreeStorage>,
    mut prefab_instances: Query<&mut PrefabInstanceData>,
    mut move_cmds: EventReader<MoveTreeCmd>,
    mut delete_cmds: EventReader<DeleteTreeCmd>,
    mut insert_cmds: EventReader<InsertTreeCmd>,
    mut spawn_prefab: EventWriter<SpawnPrefabCmd>,
    mut commands: Commands,
    mut play_sound: EventWriter<PlayTreeSoundCmd>,
    active_session: Res<ActiveSession>,
    water_raster: Res<WaterRaster>,
    mut despawn_prefab: EventWriter<DespawnPrefabCmd>,
) {
    // 1) 移动（源码行 43–74，编译偏移 0x1BC–0x4B8）：
    //    MoveTreeCmd.index → TreeStorage::get → 按下标定位 PrefabInstanceData（Query 表
    //    stride 0x14，校验 entity 后写 Vec3@+0）→ 采样水面 → PlayTreeSoundCmd(kind=1)。
    for cmd in move_cmds.read() {
        match tree_storage.get(cmd.index) {
            Some(entry) => {
                // [evidence] 按下标更新 prefab 位置；entry = {prefab_index, entity}。
                let _ = (entry, &mut prefab_instances);
                let in_water = water_sample(&water_raster, cmd.position);
                play_sound.send(PlayTreeSoundCmd {
                    kind: 1,
                    in_water,
                    flag: false,
                    height: 0.0,
                    position: cmd.position,
                });
            }
            None => {
                // 证据字符串：MoveTreeCmd references a non-existing tree
                log::error!("MoveTreeCmd references a non-existing tree");
            }
        }
    }

    // 2) 插入（源码行 74–176，编译偏移 0x5F9–0xDD1）：
    //    拖拽/名字/加载门控 → spawn_empty + TreeComponent/TreeAnimation/TreeMoveBend/
    //    TreeAmbience（经 Commands）→ prefab 名选择 → SpawnPrefabCmd → TreeStorage::insert →
    //    采样水面 → PlayTreeSoundCmd(kind=0)；条件写 DespawnPrefabCmd（槽位已占用时替换）。
    for cmd in insert_cmds.read() {
        let _ = (&active_session.state, &water_raster);
        tree_storage.insert(cmd.index);
        spawn_prefab.send(SpawnPrefabCmd {
            name: cmd.name,
            field_10: 0,
            position: cmd.position,
            field_24: 0.0,
            field_28: 0.0,
            flags: cmd.flags,
            field_30: 1.0,
            unknown_tail: [0; 0x14],
        });
        play_sound.send(PlayTreeSoundCmd {
            kind: 0,
            in_water: water_sample(&water_raster, cmd.position),
            flag: false,
            height: cmd.height,
            position: cmd.position,
        });
    }

    // 3) 删除（源码行 182–201，编译偏移 0xDD1–0x10CE）：
    //    TreeStorage::remove → Entities::contains(entity) → TreeMarkedForDeletion 命令 →
    //    PlayTreeSoundCmd(kind=2)。**本分支不发送 DespawnPrefabCmd**
    //    （由同文件 despawn_trees 系统负责实际销毁，本轮未核对）。
    for cmd in delete_cmds.read() {
        if let Some(entry) = tree_storage.remove(cmd.index) {
            let _ = entry;
            play_sound.send(PlayTreeSoundCmd {
                kind: 2,
                in_water: false,
                flag: false,
                height: 0.0,
                position: Vec3::ZERO,
            });
        }
    }
}

/// [evidence] bilinear_sample_at_world_xz<RasterBufferRef<512>> > 0.0 → 入水。
/// 依赖实现不还原。
fn water_sample(_raster: &WaterRaster, _pos: Vec3) -> bool {
    false
}
