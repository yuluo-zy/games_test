//! `rmb_autoclutter.rs` — 右键移除 auto-clutter（B 0x140976ca0，O）。
//!
//! 证据：fresh 反编译（ui_clutter-0x140976ca0）+ re-analyst-interaction：
//! - 右键命中 → **hashbrown SIMD probe**（FUN_1405b0d70 哈希 + control-byte 位掩码
//!   折叠 + 逐位键比对）→ 实体位集/组件表双重校验 → clutter 记录（0x58B 步长）；
//! - **probe 未中原版 `unwrap_failed` panic**（LAB_140976f23）——早期 Rust `log::warn +
//!   continue`，已改为 panic；
//! - **旋转 = atan2f 链 + fmod + 半角 sin/cos**（fresh 反编译）：
//!   `rot1 = atan2f(-(-2zx-2yw), -(2y²·(-1)+(z²-|q|²)·(-1)))`（|q|²=y²+x²+w²）→
//!   `base = (¬roof) ? -π/2-fmod(entry,2π) : atan2(-y_local,x_local)` →
//!   `diff = fmod(rot1-base, 2π)` → 半角 sin/cos → `rot = atan2(2·sin·cos, cos²-sin²)`
//!   （早期 Rust `|sqrt(-2wy-2x²)-(-π/2-fmod(entry))|·0.5` 公式在 fresh 无对应，已删）；
//! - 推 0x48 B `EvRightClickedAutoClutter` 事件（tag=2@0、subtype=3@+0x29、
//!   rotation@+0x2c、type@+0x3c、entity@+0x40）；
//! - 末尾 alloc 0x18 B Arc → 入历史（tag 0xd = AutoClutter，**无条件**）并调
//!   `FUN_140a86270` = `History::start_input_recording`；crate 以 [`RecordAutoClutterHistory`]
//!   事件 → 判等 defered 链再 `History::add`（机制不同、结局近似，LOGIC 附注）；
//! - 入口 `if *(in_RDX+0x18)!=0` 直达路径（emit rot1，跳过 probe）：crate 事件模型
//!   的命中实体即已知目标，该直达面由事件承载，登记 I。
//!
//! crate 无原生 ECS 实体/组件表机制（`DecoratorStorage` 在 shared building modules，宿主提供
//! 实体机制），本实现以 [`AutoClutterEntityIndex`]（hashbrown 表，宿主维护记录）承载
//! 右键 probe 目标；probe 用 hashbrown `raw_entry`（真实哈希表键查找，与 FUN_1405b0d70
//! 语义对应）。
use bevy_ecs::prelude::*;

use crate::resources::history::auto_clutter_edit::AutoClutterEdit;
use crate::resources::history::EditType;
use crate::resources::history::History;
use crate::systems::mode_manager::AppMode;
use crate::systems::mode_manager::ClutterMeshTy;

use super::AutoClutterRecord;

/// 右键命中 auto-clutter（I：宿主从 UI 悬停/射线解析出实体 id）。
#[derive(Event, Clone, Copy, Debug, PartialEq, Eq)]
pub struct RmbAutoClutterHit(pub u64);

/// 入口直达路径（O：`*(int*)(in_RDX+0x18) != 0` → 跳过 probe，直接使用当前
/// 命中记录 + 原始四元数）。crate 以宿主提供的可选记录承载（I：原生 in_RDX/+0x18
/// 判别与 +0x20/+0x28 载荷由 UI 悬停资源写入）。
#[derive(Resource, Clone, Copy, Debug, Default)]
pub struct RmbAutoClutterDirect(pub Option<AutoClutterRecord>);

/// 右键转换产生的 0x48 B spawn 事件（O 布局：tag=2@0、subtype=3@+0x29、
/// rotation@+0x2c、type@+0x3c、entity@+0x40；payload 其余字段语义 I）。
#[derive(Event, Clone, Copy, Debug)]
pub struct EvRightClickedAutoClutter {
    pub entity: u64,
    pub ty: ClutterMeshTy,
    pub position: glam::Vec3,
    pub rotation: f32,
}

/// `CircularF32::as_f32`（O：0x1416f3210）= 角度归约到 [0, 2π)。
pub fn fmod_2pi(angle: f32) -> f32 {
    let two_pi = core::f32::consts::TAU;
    let mut reduced = angle % two_pi;
    if reduced < 0.0 {
        reduced += two_pi;
    }
    reduced
}

/// 旋转公式（O，fresh 反编译 0x140976ca0）：atan2f 链 + fmod + 半角 sin/cos。
///
/// 机械复刻（含 `-0.0`/`-1.0` 常数与 `atan2f(-yterm,-xterm)` 次序）：
/// 1. `q2 = y²+x²+w²`、`neg2y = -2y`；
/// 2. `rot1 = atan2f(-((2z)(-x)+(-2y)w+(z²-q2)(-0.0)),
///                   -((2z)(0)+(-2y)y+(z²-q2)(-1.0)))`；
/// 3. `base = (¬roof) ? -π/2 - fmod(entry,2π) : atan2(-y_local,x_local)`；
/// 4. `diff = fmod(rot1-base, 2π)`；
/// 5. 半角：`sin_half=sin(diff/2)`、`cos_half=cos(diff/2)`；
/// 6. `rot = atan2(2·sin_half·cos_half, cos_half²-sin_half²)`（双角恢复 = diff）。
pub fn get_clutter_rot_in_roof_space(
    w: f32,
    x: f32,
    y: f32,
    z: f32,
    entry_angle: f32,
    roof_space: bool,
    local_x: f32,
    local_y: f32,
) -> f32 {
    // rot1（fresh `LAB_140976f99` atan2f 链）。
    let q2 = y * y + x * x + w * w;
    let neg2y = (y * -1.0 + x * -0.0 + w * -0.0) * 2.0;
    let rot1 = (-((z + z) * (x * -1.0 - y * -0.0) + neg2y * w + (z * z - q2) * -0.0))
        .atan2(-((z + z) * (w * -0.0 - x * -0.0) + neg2y * y + (z * z - q2) * -1.0));
    // entry base（probe 路径）。
    let base = if !roof_space {
        -core::f32::consts::FRAC_PI_2 - fmod_2pi(entry_angle)
    } else {
        (-local_y).atan2(local_x)
    };
    // diff = fmodf(rot1 - base, 2π)；Rust `%` = fmodf（截断余数）。
    let diff = (rot1 - base) % core::f32::consts::TAU;
    let sin_half = (diff * 0.5).sin();
    let cos_half = (diff * 0.5).cos();
    // 半角双角恢复：atan2(2 sin cos, cos²-sin²)。
    (2.0 * sin_half * cos_half).atan2(cos_half * cos_half - sin_half * sin_half)
}

/// `handle_rmb_autoclutter` @0x140976ca0（逐语句重建）。
///
/// 入口直达（`RmbAutoClutterDirect` 有值）→ 跳过 probe；否则 hashbrown entity probe
/// （未中 `unwrap_failed` panic）→ 旋转（atan2f 链 + fmod + 半角）→ 0x48B
/// `EvRightClickedAutoClutter`（@+0x29=3）→ `History::start_input_recording` →
/// **直接 `History::add(tag 0xd)`**（无条件；fresh alloc 0x18 `{1,1,id}`）。
pub fn handle_rmb_autoclutter<S: Send + Sync + 'static>(
    mut hits: EventReader<RmbAutoClutterHit>,
    direct: Res<RmbAutoClutterDirect>,
    index: Res<super::AutoClutterEntityIndex>,
    mode: Res<AppMode>,
    mut history: ResMut<History<S>>,
    mut right_clicked: EventWriter<EvRightClickedAutoClutter>,
) {
    for hit in hits.read() {
        // 入口门（O：`*(int*)(in_RDX+0x18) != 0` → LAB_140976f99，跳过 probe）。
        let record = match direct.0 {
            Some(record) => record,
            None => {
                // hashbrown raw_entry probe（O：FUN_1405b0d70 + control-byte probe 语义）。
                let Some(record) = index.0.raw_entry().from_key(&hit.0).map(|entry| *entry.1)
                else {
                    // **probe 未中原版 `option::unwrap_failed` panic**（LAB_140976f23）。
                    Option::<AutoClutterRecord>::None.unwrap();
                    unreachable!()
                };
                record
            }
        };
        // 旋转（O：atan2f 链 + fmod + 半角 sin/cos）。
        let rotation = get_clutter_rot_in_roof_space(
            record.w,
            record.x,
            record.y,
            record.z,
            record.entry_angle,
            record.roof_space,
            record.local_x,
            record.local_y,
        );
        // 0x48B EvRightClickedAutoClutter（@+0x29 = 3）。
        right_clicked.write(EvRightClickedAutoClutter {
            entity: hit.0,
            ty: record.ty,
            position: record.position,
            rotation,
        });
        // O：`History::start_input_recording()`（0x140a86270）→ alloc 0x18 `{1,1,id}` →
        // `History::add`（tag 0xd），**无条件、无录音门**。
        let _ = history.start_input_recording(mode.native_tag());
        let _ = history.add(EditType::AutoClutter(std::sync::Arc::new(
            AutoClutterEdit::Delete { id: hit.0 },
        )));
    }
}
