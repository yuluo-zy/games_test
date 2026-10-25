//! cursor_terrain_raycast.rs — 光标地形射线（WP8 恢复 + ① T1 契约补全）。
//!
//! 源：B skeleton `resources/cursor_terrain_raycast.rs`。
//! 反编译：`CursorTerrainRaycast::position`@0x140a12650（行 35）：`(*tag & 1)!=0` 时返回
//! 位置 Vec3（+0x04/+0x0c），否则 {0,0,0}。方法行号：`update`@20、`new`@39。
//! 布局（O，PDB procs.tsv 行 49396 + 写入者 0x1408ef320，① contract
//! round4/contracts/t1-garden_audio-130-gate-CONTRACT.md §2）：**32B**
//! `{ tag:u32@0x00, pos:Vec3@0x04, delta:Vec3@0x10, speed:f32@0x1c }`。
//!
//! 写入者 `mouse_terrain_raycast`（0x1408ef320，terrain-water/decomp）逐槽写入：
//! `tag` 命中置 1（未命中分支不写，保持 0）；`pos` = 世界位置（+4/+8/+0xc）；
//! `delta` = 本帧位置差（+0x10/0x14/0x18，仅前帧有效才写）；`speed`（+0x1c）=
//! `len(delta) / [param_7+0x58]`（param_7 资源距离场，除数身份待取证）。
//!
//! 消费面：`garden_audio`/`wall_audio_fmod`/`terrain_stairs`/`tree`/`water` 读
//! `[+0x1c]` = editing volume（= speed，非独立资源）；`ui_draw_terrain`、
//! `ivy_spawner`、render draw_passes 读 `tag`/`pos`。

#![allow(dead_code, unused_imports)]

use bevy_ecs::resource::Resource;
use glam::Vec3;

/// 光标地形射线（O 布局 32B；`tag` 为 Some 标记，bit 0 = 命中）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct CursorTerrainRaycast {
    /// O：原生 `u32` @+0x00；bit 0 = 悬停/命中标记（写入者 0x1408ef320 命中置 1）。
    pub tag: u32,
    /// O：世界位置 @+0x04（x@+4、y@+8、z@+0xc）。
    pub pos: Vec3,
    /// O：本帧位置差 @+0x10（+0x10/0x14/0x18；仅前帧有效才写）。
    pub delta: Vec3,
    /// O：编辑速度 @+0x1c（= `len(delta) / [param_7+0x58]`；除数资源身份待取证）。
    /// 共享 editing-volume 源，被 garden_audio / wall_audio_fmod / terrain_stairs /
    /// tree / water 读取（`+0x1c`）。
    pub speed: f32,
}

impl CursorTerrainRaycast {
    /// 新建（O 名；`new`@39）。
    pub fn new() -> Self {
        Self {
            tag: 0,
            ..Default::default()
        }
    }

    /// 更新（O 名；`update`@20）。输入语义 U；delta/speed 由 `mouse_terrain_raycast`
    /// 写入者（0x1408ef320）逐帧维护，不在本方法内。
    pub fn update(&mut self, _hit: Option<Vec3>) {
        match _hit {
            Some(pos) => {
                self.tag = 1;
                self.pos = pos;
            }
            None => {
                self.tag = 0;
                self.pos = Vec3::ZERO;
            }
        }
    }

    /// 返回位置（O：`position`@0x140a12650 读 +4/+0xc）。
    pub fn position(&self) -> Vec3 {
        if (self.tag & 1) != 0 {
            self.pos
        } else {
            Vec3::ZERO
        }
    }

    /// 射线距离（O：ui_draw_terrain @0x140a9fdc0 读 `+8`；与 `pos.y` 同槽）。
    pub fn distance(&self) -> f32 {
        self.pos.y
    }
}
