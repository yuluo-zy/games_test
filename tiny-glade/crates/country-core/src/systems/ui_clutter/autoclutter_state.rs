//! `autoclutter_state.rs` — AutoClutter 自动摆放状态与记录布局（B `system_clutter::autoclutter`，
//! O 2026-10-17 coregap-1 fresh 反编译 `add_region` 0x140ee3fd0 / `mark_dirty` 0x140ee48c0 /
//! `flush_candidates_pending_removal` 0x140ee4bf0）。
//!
//! 证据：区域表 = hashbrown `RawTable<GlobalId, AutoClutterRegion>` @+0x60（表项 stride 0x48 =
//! `{key GlobalId@+0, AutoClutterRegion@+8}`）；pending 待移除索引 `Vec<u16>` @+0x30 cap /
//! +0x38 data / +0x40 len（哨兵 0xffff、有效 `< 0xfffe`）；epoch 字节 @+0x90。
//! 记录布局与 `interaction/clutter/RESTORE.md` §类型合同一致（region 0x40B、spawn 0x68、
//! candidate 0x98）。

use bevy_ecs::resource::Resource;
use glam::Vec3;

/// 自动摆放区域记录（O：0x40B 值 + 8B 键 = 0x48B 表项；字段序见模块注释）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct AutoClutterRegion {
    /// @+0：种类判别（O：add_region 以 `== 4` 判 Arc 载荷释放）。
    pub kind: u32,
    /// @+8/@+0x10（U：语义待证）。
    pub a: u64,
    pub b: u64,
    /// @+0x18..+0x30：7×f32（O：add_region `is_equivalent` 逐 f32 比较区 = 区域几何
    /// 中心/基向量；`iter_world_space_rect_cells` 输入）。
    pub f: [f32; 7],
    /// @+0x34：i32（O：`is_equivalent` 比较）。
    pub i: i32,
    /// @+0x38（O：`is_equivalent` 比较）。
    pub u8a: u8,
    /// @+0x39（O：`is_equivalent` 比较）。
    pub u8b: u8,
    /// @+0x3a：候选索引（O：哨兵 0xffff、有效 `< 0xfffe`）。
    pub candidate_index: u16,
    /// @+0x3c：epoch（O：add_region/mark_dirty 写当前帧 epoch）。
    pub epoch: u8,
    /// @+0x3d..+0x3f：pad。
    pub pad: [u8; 3],
}

impl AutoClutterRegion {
    /// `is_equivalent`（O：add_region 逐 7×f32（IEEE 序，NaN 不相等）+ i32 + 2×u8，
    /// **不含** candidate_index/epoch）。
    pub fn is_equivalent(&self, other: &Self) -> bool {
        self.f.iter()
            .zip(&other.f)
            .all(|(a, b)| a.to_bits() == b.to_bits())
            && self.i == other.i
            && self.u8a == other.u8a
            && self.u8b == other.u8b
    }
}

/// 自动摆放生成记录（O：0x68B；字段序见 clutter RESTORE §类型合同）。
#[derive(Clone, Debug, Default)]
#[repr(C)]
pub struct AutoClutterSpawn {
    /// @+0：kind u64。
    pub kind: u64,
    /// @+8：shared 载荷（U）。
    pub shared: u64,
    /// @+0x10..+0x28：3×u64 载荷区（U）。
    pub payload: [u64; 3],
    /// @+0x30：marker 字节。
    pub marker: u8,
    /// @+0x34：u32。
    pub u34: u32,
    /// @+0x38/+0x40/+0x48：3×u64。
    pub u38: u64,
    pub u40: u64,
    pub u48: u64,
    /// @+0x50/+0x54/+0x58/+0x5c：4×u32。
    pub u50: u32,
    pub u54: u32,
    pub u58: u32,
    pub u5c: u32,
    /// @+0x60：scale。
    pub scale: f32,
    /// @+0x64：pad。
    pub pad: u32,
}

/// Native `system_clutter::autoclutter::AutoClutterObject` record, **0x68 B**,
/// key `u64 @+0x40` (fresh 2026-10-17).
///
/// Anchor: the `HashMap<u64, &AutoClutterObject>` construction in
/// `manage_autoclutter_entities` monomorphizes to
/// `alloc::...::from_iter` 0x142612aa0:
/// `IMUL RAX,R15,0x68` (stride) and
/// `MOV RDX,[RDI + RAX + 0x40]` (the map key is the object's `+0x40` qword).
///
/// Only the key offset and the total size are proven; every other byte stays
/// opaque (`[u8; 0x40]` + `[u8; 0x20]`) rather than inventing fields. The live
/// implementation of this type belongs to the `system-clutter` crate, which
/// depends on `country-core`; this owner is the country-core-side contract.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AutoClutterObject {
    /// +0x00..+0x3f: unproven payload bytes.
    pub payload: [u8; 0x40],
    /// +0x40: map key (`HashMap<u64, &AutoClutterObject>`).
    pub key: u64,
    /// +0x48..+0x67: unproven payload bytes.
    pub tail: [u8; 0x20],
}

impl Default for AutoClutterObject {
    fn default() -> Self {
        Self {
            payload: [0; 0x40],
            key: 0,
            tail: [0; 0x20],
        }
    }
}

impl AutoClutterObject {
    /// Native record size (`IMUL ...,0x68` in 0x142612aa0).
    pub const NATIVE_SIZE: usize = 0x68;

    /// The `+0x40` map key.
    pub fn key(&self) -> u64 {
        self.key
    }
}

/// 自动摆放候选记录（O：0x98B；`+0x30 byte==6 强制接受`、`+0x68..+0x88 Rectangle3d`、
/// `+0x88 region_key`）。
#[derive(Clone, Debug)]
#[repr(C)]
pub struct AutoClutterCandidate {
    /// @+0..+0x2f：记录前段（U）。
    pub head: [u64; 6],
    /// @+0x30：强制接受判别（O：`== 6` 强制）。
    pub force: u8,
    /// @+0x31..+0x67：U。
    pub gap: [u8; 0x37],
    /// @+0x68..+0x88：Rectangle3d（中心 + 半尺寸）。
    pub rect: Rectangle3d,
    /// @+0x88：region key（O：flush 换入候选键）。
    pub region_key: u64,
}

impl Default for AutoClutterCandidate {
    fn default() -> Self {
        Self {
            head: [0; 6],
            force: 0,
            gap: [0; 0x37],
            rect: Rectangle3d::default(),
            region_key: 0,
        }
    }
}

/// 3D 矩形（O：`process_autoclutter_candidates` 的 `Rectangle3d::overlaps_rectangle3d`
/// 输入面）。
#[derive(Clone, Copy, Debug, Default, PartialEq)]
#[repr(C)]
pub struct Rectangle3d {
    pub center: Vec3,
    pub half_extents: Vec3,
}

/// 自动摆放状态（O：B 二进制布局——懒网格区域表 @+0x60、pending Vec<u16> @+0x30..0x48、
/// epoch @+0x90；其余字段语义 U，以注释登记。**注意**：系统层实现为 `system-clutter`
/// crate 的语义等价字段承载，本类型供上层按原二进制契约迁移）。
#[derive(Resource, Debug, Default)]
pub struct AutoClutterState {
    /// @+0x08/+0x10：脏格/矩形来源切片（O：`iter_world_space_rect_cells` 的输入面）。
    pub source: Vec<u64>,
    /// @+0x30/+0x38/+0x40：待移除候选索引 `Vec<u16>`（O：哨兵 0xffff、`<0xfffe` 有效）。
    pub pending_removal: Vec<u16>,
    /// @+0x60..+0x80：区域表 `RawTable<GlobalId, AutoClutterRegion>`（hashbrown；O 表项
    /// 0x48B）。crate 以 `hashbrown::HashMap<u64, AutoClutterRegion>` 语义承载（键序 I）。
    pub regions: hashbrown::HashMap<u64, AutoClutterRegion>,
    /// @+0x90：epoch（O：add_region/mark_dirty/flush 读写；`set_current_tick` 逐帧推进）。
    pub epoch: u8,
}

impl AutoClutterState {
    /// 候选索引哨兵（O：0xffff）。
    pub const CANDIDATE_NONE: u16 = 0xffff;
    /// 有效候选索引上界（O：`< 0xfffe`）。
    pub const CANDIDATE_MAX: u16 = 0xfffe;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 0x142612aa0: stride `0x68`, key load at `+0x40`.
    #[test]
    fn auto_clutter_object_is_sixty_eight_bytes_with_key_at_0x40() {
        assert_eq!(
            core::mem::size_of::<AutoClutterObject>(),
            AutoClutterObject::NATIVE_SIZE
        );
        assert_eq!(core::mem::size_of::<AutoClutterObject>(), 0x68);
        assert_eq!(core::mem::offset_of!(AutoClutterObject, key), 0x40);
        let object = AutoClutterObject {
            key: 0x1234,
            ..Default::default()
        };
        assert_eq!(object.key(), 0x1234);
    }
}
