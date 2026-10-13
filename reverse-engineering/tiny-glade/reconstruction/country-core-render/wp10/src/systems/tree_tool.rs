//! `systems::tree_tool` — 树工具系统（B 骨架 `systems/tree-tool` crate）。
//!
//! 反编译（Ghidra）：`process_tree_cmds` 0x142267430 / `despawn_trees` 0x1422686d0 /
//! `hide_trees` 0x14226a260 / `tree_move_bend` 0x1422689c0。
//! 命令 payload 候选复用 `reconstruction/tree-tool`（reva/radare2 反汇编；B 骨架名核对）。

#![allow(dead_code, unused_imports)]

use bevy_ecs::entity::Entity;
use bevy_ecs::event::Event;
use bevy_ecs::query::With;
use bevy_ecs::system::{Commands, Query};
use glam::Vec3;

use crate::components::{TreeAnimation, TreeMarkedForDeletion};
use crate::resources::tree_storage::{MoveTreeCmd, TreeStorage};
use crate::systems::support::GameTime;
use crate::typed_gap::Gap;

// ---- 命令事件（payload 候选来自 tree-tool 反汇编；字段 U 处为 typed gap）----

/// 删除树命令（history::tree_edit，serde；payload 8B index u64，I）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DeleteTreeCmd {
    pub index: u64,
}

/// 插入树命令（history::tree_edit，serde；payload 64B，字段候选见 reconstruction/tree-tool）。
#[derive(Event, Clone, Debug, Default)]
pub struct InsertTreeCmd {
    pub index: u64,
    pub name: String,
    pub position: Vec3,
    pub height: f32,
    pub flags: u32,
    /// 其余字段 U。
    pub _gap: Gap,
}

/// 生成预制体命令（systems::prefab；payload 候选）。
#[derive(Event, Clone, Debug, Default)]
pub struct SpawnPrefabCmd {
    pub name: String,
    pub position: Vec3,
    /// 其余字段 U。
    pub _gap: Gap,
}

/// 销毁预制体命令（systems::prefab）。
#[derive(Event, Clone, Copy, Debug)]
pub struct DespawnPrefabCmd {
    pub entity: Entity,
}

/// 树音效事件（systems::audio::tree_audio；kind 0/1/2 = insert/move/delete，I）。
#[derive(Event, Clone, Copy, Debug)]
pub struct PlayTreeSoundCmd {
    pub kind: u8,
    pub in_water: bool,
    pub height: f32,
    pub position: Vec3,
}

/// `process_tree_cmds`（0x142267430，O）：消费 `MoveTreeCmd`/`InsertTreeCmd`/`DeleteTreeCmd` →
/// TreeStorage get/insert/remove + `SpawnPrefabCmd`/`DespawnPrefabCmd` + `PlayTreeSoundCmd`。
/// 三段处理顺序 Move → Insert → Delete（tree-tool 反汇编分支头 0x142267640/0x142267a29/0x142268201）。
#[allow(clippy::too_many_arguments)]
pub fn process_tree_cmds(
    tree_storage: &mut TreeStorage,
    mut move_cmds: bevy_ecs::event::EventReader<MoveTreeCmd>,
    mut delete_cmds: bevy_ecs::event::EventReader<DeleteTreeCmd>,
    mut insert_cmds: bevy_ecs::event::EventReader<InsertTreeCmd>,
    mut spawn_prefab: bevy_ecs::event::EventWriter<SpawnPrefabCmd>,
    mut commands: Commands,
    mut play_sound: bevy_ecs::event::EventWriter<PlayTreeSoundCmd>,
    mut despawn_prefab: bevy_ecs::event::EventWriter<DespawnPrefabCmd>,
) {
    // 1) Move：get(cmd.index) 命中 → 更新位置 + PlayTreeSoundCmd(kind=1)；未命中 log error。
    for cmd in move_cmds.read() {
        match tree_storage.get_mut(cmd.index) {
            Some(_entry) => {
                play_sound.write(PlayTreeSoundCmd {
                    kind: 1,
                    in_water: false,
                    height: 0.0,
                    position: cmd.position,
                });
            }
            None => {
                log::error!("MoveTreeCmd references a non-existing tree");
            }
        }
    }

    // 2) Insert：TreeStorage::insert(index) + SpawnPrefabCmd + PlayTreeSoundCmd(kind=0)。
    for cmd in insert_cmds.read() {
        tree_storage.insert(cmd.index);
        spawn_prefab.write(SpawnPrefabCmd {
            name: cmd.name.clone(),
            position: cmd.position,
            _gap: Gap,
        });
        play_sound.write(PlayTreeSoundCmd {
            kind: 0,
            in_water: false,
            height: cmd.height,
            position: cmd.position,
        });
    }

    // 3) Delete：remove(index) 命中 → TreeMarkedForDeletion（经 Commands）+ PlayTreeSoundCmd(kind=2)。
    for cmd in delete_cmds.read() {
        if tree_storage.remove(cmd.index).is_some() {
            let _ = &mut commands;
            play_sound.write(PlayTreeSoundCmd {
                kind: 2,
                in_water: false,
                height: 0.0,
                position: Vec3::ZERO,
            });
        }
    }

    let _ = &mut despawn_prefab;
}

/// `despawn_trees`（0x1422686d0，O）：`Query<(Entity,&TreeAnimation,&TreeMarkedForDeletion)>`；
/// `progress < 0.01`（严格小于，NaN 不销毁）→ `DespawnPrefabCmd`。
pub fn despawn_trees(
    query: Query<(Entity, &TreeAnimation, &TreeMarkedForDeletion)>,
    mut despawn_prefab: bevy_ecs::event::EventWriter<DespawnPrefabCmd>,
) {
    for (entity, anim, _marked) in query.iter() {
        if anim.despawn_progress < 0.01 {
            despawn_prefab.write(DespawnPrefabCmd { entity });
        }
    }
}

/// `hide_trees`（0x14226a260，O）：消费 `HideTreesNextToWallCmd`/`HideTreesinAreaCmd`，
/// 隐藏/显示 TreeStorage 中树（掩码细节 U）。
pub fn hide_trees(
    _storage: &TreeStorage,
    _near_wall: bevy_ecs::event::EventReader<crate::resources::tree_storage::HideTreesNextToWallCmd>,
    _in_area: bevy_ecs::event::EventReader<crate::resources::tree_storage::HideTreesinAreaCmd>,
) {
}

/// `tree_move_bend`（0x1422689c0，O）：拖拽弯曲插值（动画状态机，公式 U）。
pub fn tree_move_bend(_time: &GameTime, _query: Query<Entity, ()>) {}

// ---- UI 命令生产（名 O，行为 I）----
pub fn ui_place_tree(_commands: Commands) {}
pub fn ui_move_tree(_commands: Commands) {}
pub fn ui_delete_tree(_commands: Commands) {}
