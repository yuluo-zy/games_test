//! add_autoclutter_on_windows（B 主基线；0x1422cd7d0，size 1622）。
//!
//! 证据（clutter-extra-analysis.md §on_windows，radare2 RNE oracle 全函数反汇编）：
//! - 输入候选 Vec（stride **0x50**）：`+0x00` byte = 种类（跳表索引）；`CMP ECX,6; JZ skip`
//!   （==6 直接跳过，与 process_candidates 强制路径同判据）；`+0x01`/`+0x04`/`+0x08`/
//!   `+0x10`(16B 位置)/`+0x20`/`+0x28`/`+0x40`/`+0x48` 字段。
//! - 跳表 0x143042370（6 项 disp32）→ 分支差异只在宿主源与标志：
//!   | idx | 宿主源 |
//!   |---|---|
//!   | 0 | base+0xf0 |
//!   | 1 | base Vec（+8/+0x10，以候选+0x08 为下标） |
//!   | 2 | base+0xf8（标志翻转：`NOT r12b`） |
//!   | 3 | base+0x1a8 |
//!   | 4 | base+0x1b0 |
//!   | 5 | base+0x1b8 |
//! - 6 分支共享同一向量数学（候选位置 XMM8 绕常量 0x1429cd0c0=(0,0,1,0) 与
//!   0x142925904=1.0 做旋转/投影，含 `2*p.z*p` 反射项）。
//! - 写块 0x1422cd8c0：type=5、位置 16B、+0x18=1、+0x20=候选+0x48、+0x30=标志
//!   （2 或 !候选[1]）、+0x38=宿主子结构首 qword、+0x40=候选+0x40、+0x48=候选+0x20、
//!   +0x50=候选+0x28、+0x54=位置 x/z、+0x5c=y、+0x60=1.0。

use bevy_ecs::prelude::*;
use glam::Vec3;

use super::{AutoClutterSpawn, SpawnShared, AutoClutterState};

/// on_windows 输入候选（stride 0x50）。
#[derive(Clone, Debug)]
pub struct OnWindowCandidate {
    /// +0x00：种类字节（跳表索引 0–5；6 跳过）。
    pub kind: u8,
    /// +0x01：标志字节（index 2 翻转）。
    pub flag: u8,
    /// +0x04。
    pub f04: u32,
    /// +0x08：宿主 Vec 下标（index 1 分支用）。
    pub host_index: u64,
    /// +0x10：16B 候选位置（XMM8）。
    pub position: Vec3,
    /// +0x20。
    pub f20: u64,
    /// +0x28。
    pub f28: u64,
    /// +0x40。
    pub f40: u64,
    /// +0x48。
    pub f48: u64,
}

/// 窗宿主子结构（6 分支源；宿主装配时填充）。
#[derive(Clone, Debug, Default)]
pub struct WindowHostSub {
    /// 首 qword（墙/窗 id；写输出 +0x38）。
    pub id: u64,
    /// 子结构数据（+0xf0/+0xf8/+0x1a8/+0x1b0/+0x1b8 等段；宿主按分支索引）。
    pub substructure: [Vec3; 2],
}

/// 窗宿主（index 1 分支的 Vec 宿主）。
#[derive(Clone, Debug, Default)]
pub struct OnWindowHostVec {
    pub entries: Vec<OnWindowCandidate>,
}

/// on_windows 输入（资源；宿主装配）。
#[derive(Resource, Clone, Debug, Default)]
pub struct OnWindowInput {
    pub candidates: Vec<OnWindowCandidate>,
    pub hosts: [WindowHostSub; 6],
}

/// add_autoclutter_on_windows（0x1422cd7d0）：按候选种类字节经跳表分派选窗宿主
/// 子结构，同一旋转投影计算落点，直接写 type=5 输出候选（0x68 stride）。
pub fn add_autoclutter_on_windows(
    mut state: ResMut<AutoClutterState>,
    input: Res<OnWindowInput>,
) {
    for candidate in &input.candidates {
        let kind = candidate.kind;
        if kind == 6 {
            continue; // ==6 直接跳过（O）。
        }
        let idx = (kind as usize).min(5);
        let host = &input.hosts[idx];
        // 旋转/投影（O：候选位置绕常量 (0,0,1,0) 与 1.0，含 2*p.z*p 反射项）。
        let p = candidate.position;
        let reflect = Vec3::new(
            2.0 * p.z * p.x,
            2.0 * p.z * p.y,
            2.0 * p.z * p.z - p.length_squared(),
        );
        let pos = Vec3::new(reflect.x, candidate.position.y, reflect.z);
        let marker = if kind == 2 { !candidate.flag & 1 } else { 2 };
        state.spawn_candidates.push(AutoClutterSpawn {
            kind: 5,
            shared: SpawnShared::Prefab("window".to_string()),
            f18: 1,
            key: candidate.f48,
            marker,
            position: pos,
            host_id: host.id,
            f48: candidate.f20,
            f50: candidate.f28,
            f54: candidate.f40 as u32,
            f5c: candidate.f04,
            scale: 1.0,
        });
    }
}
