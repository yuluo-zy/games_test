//! tree-tool：tree_move_bend 业务恢复草稿（B 主基线，反汇编级证据）
//!
//! 依据：evidence/multi-baseline/tree-tool-rest-analysis.md（2026-10-13 reva/radare2
//! 精读 0x1422689c0，229 条指令）。
//!
//! 硬证据：
//! - validate_param_unsafe 包装器 0x141bd92a0 签名 =
//!   Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>, Res<GameTime>。
//! - 反汇编：position()→目标 xz；time_continuous_lerp 两次（current,target,delta,
//!   rate=5.0）写 TreeMoveBend.x/y；dist>=0.1 时 step=min(dist,5.0)*(3.22/dist)*delta，
//!   bend=target+step；截断 step 与 PrefabInstanceData+0x24/+0x28 比较，变则写
//!   tick + 打包 u64。
//! - 常量（radare2 确认）：0.1 @0x142925984、5.0 @0x142925988、3.22 @0x142925904。
//! - time_continuous_lerp（0x140c904e0）= target+(current-target)*expf(-delta*rate)。
//!
//! 依赖复用：bevy_ecs 0.16 直接复用。PrefabInstanceData 同 recalc 文件定义。

use bevy_ecs::change_detection::Res;
use bevy_ecs::prelude::*;
use bevy_ecs::query::Query;
use glam::Vec2;

/// 树弯曲组件（country_core::components::TreeMoveBend）。
/// 本轮证据：Vec2（stride 8）@+0/+4，每帧 lerp/step 写回。语义 unknown
/// （疑似树移动时的滑移/弯曲量，向 PrefabInstanceData 位置缓动）。
pub struct TreeMoveBend {
    pub x: f32,
    pub y: f32,
}

/// 预制体实例数据组件（同 recalc 文件；size 0x30）。
/// tree_move_bend 写 position @+0（目标）与 +0x24/+0x28（step 变更检测）。
pub struct PrefabInstanceData {
    /// 位置 Vec3 @+0；x/z 作 TreeMoveBend 缓动目标。
    pub position: Vec3,
    /// 其余字段（@+0xc..+0x24、+0x2c..）unknown。
    pub unknown: [u8; 0x18],
    /// [evidence] f32 @+0x24：上次写入的截断 step_x（变更检测）。
    pub step_x: f32,
    /// [evidence] f32 @+0x28：上次写入的截断 step_y（变更检测）。
    pub step_y: f32,
}

/// 树弯曲缓动系统。
///
/// 签名来自 validate_param_unsafe 包装器（硬证据）。PDB size 1019（0x3FB），
/// 源文件 tree_move_bend.rs 8–42 行。
pub fn tree_move_bend(
    mut query: Query<(&mut TreeMoveBend, &mut PrefabInstanceData)>,
    time: Res<GameTime>,
) {
    const LERP_RATE: f32 = 5.0;   // [evidence] 0x142925988
    const MIN_DIST: f32 = 0.1;    // [evidence] 0x142925984
    const STEP_RATE: f32 = 3.22;  // [evidence] 0x142925904

    for (mut bend, mut instance) in &mut query {
        // 1) position()（0x140ad51a0）→ 目标 x/z。
        let target = Vec2::new(instance.position.x, instance.position.z);

        // 2) time_continuous_lerp(current, target, delta, rate) 两次 → bend.x/.y。
        //    （0x140c904e0 = target + (current-target)*expf(-delta*rate)）
        let delta = time.delta_seconds;
        bend.x = time_continuous_lerp(bend.x, target.x, delta, LERP_RATE);
        bend.y = time_continuous_lerp(bend.y, target.y, delta, LERP_RATE);

        // 3) dist >= 0.1 时 step 逼近目标。
        //    [evidence] step_vec = min(5.0, dist) * (3.22/dist) * delta_vec
        //    （反汇编 0x142268d09..0x142268d2a：XMM2=3.22/dist；XMM3=min(5,dist)；
        //     XMM0 = min(5,dist) * delta_vec * (3.22/dist)；ADDPS 到 target 写回）。
        let dist = Vec2::new(bend.x - target.x, bend.y - target.y).length();
        if dist >= MIN_DIST {
            let delta_vec = Vec2::new(bend.x - target.x, bend.y - target.y);
            let step_vec = delta_vec * (dist.min(5.0) * STEP_RATE / dist);
            bend.x = target.x + step_vec.x;
            bend.y = target.y + step_vec.y;

            // 4) step_vec 分量（MOVD 取 f32 位）与 PrefabInstanceData+0x24/+0x28 比较，
            //    变则写 tick + 打包 u64
            //    （反汇编 0x142268d2d..0x142268a90；UCOMISS 与存储值比较；
            //     变则 [RDI+0x24] = step_x|step_y<<32）。字段名 unknown。
            let step_x = step_vec.x;
            let step_y = step_vec.y;
            if instance.step_x != step_x || instance.step_y != step_y {
                instance.step_x = step_x;
                instance.step_y = step_y;
                // [evidence] 另写 bevy change tick 到 PrefabInstanceData tick 列。
            }
        }
    }
}

/// [evidence] 0x140c904e0；`utils::time_continuous_lerp`，依赖实现不还原。
fn time_continuous_lerp(current: f32, target: f32, delta: f32, rate: f32) -> f32 {
    target + (current - target) * (-delta * rate).exp()
}

// ---- country_core::resources::time ----

/// 游戏时间资源（同 replay_tree）。f32 @+0x28 = delta。
pub struct GameTime {
    pub _private: [u8; 0x28],
    pub delta_seconds: f32,
}
