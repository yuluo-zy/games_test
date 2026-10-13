//! sheep_terrain::sheep_walk_on_terrain 地形行走（B 主基线，反汇编级证据草稿）。
//!
//! 独立证据草稿：完整 bevy 系统见 `evidence/multi-baseline/critters-behavior-analysis.md`；
//! 本文件只落反汇编钉死的状态机与竖向弹簧-阻尼核心（bevy-free，glam 即可）。
//!
//! 证据源：reva worker `critter-owner` 反汇编/反编译 0x14231bd80（PDB size 1556，
//! 源码 sheep_terrain.rs 15–80）；radare2 RNE oracle 抽点 + .data 常量 hexdump 证实。
//!
//! 时序/常量（证据）：`dt = *Res<Time> + 0x28`；`smooth = 1 - exp(-3.0*dt)`
//! （0x142ad58e8 = -3.0f）；0x142925904 = 1.0f；+4.0 身位偏移；行走增益 10.0、
//! 浮空增益 20.0、下陷 ×3 修正；clamp 行走 [-2,2]/[-5,5]、浮空 [-0.3,0.3]；
//! 阈值 -0.05 转地面态；0.142a7b658 = 130.0f（采样参数）。
//!
//! 地形高度来源：`TerrainHeightsData::sample_at_world_xz<TerrainHeightSsbo>`
//! （0x141246fb0，@0x989）—— 建筑/地形查询衔接点（依赖复用，不重实现）。

/// 羊地形状态记录（8B 槽）：`state u32 @ +0`、`f32 @ +4`（@+4 语义 unknown）。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TerrainFollowerState(pub u32);

/// 竖向速度/差记录（12B 槽）：仅 @+4 被读写（反汇编证据）；其余字节 unknown。
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct VerticalDelta(pub f32);

/// 行走状态码（反汇编 switch 证据）：2=地面吸附；4=过渡；6=浮空；3=panic 不可达；
/// 0/1/5=行走（default 分支）。
#[allow(dead_code)]
pub const STATE_GROUNDED: u32 = 2;
#[allow(dead_code)]
pub const STATE_TRANSITION: u32 = 4;
#[allow(dead_code)]
pub const STATE_FLOATING: u32 = 6;

const OFFSET: f32 = 4.0;
const WALK_GAIN: f32 = 10.0;
const FLOAT_GAIN: f32 = 20.0;

/// 竖向弹簧-阻尼核心（sheep_walk_on_terrain 0x14231bd80 每实体，源码 27–80）。
///
/// 证据（反汇编 @0x1030 起 switch(state)）：
/// - 2：`pos.y = terrain_h`（吸附）。
/// - 4：`pos.y = time_continuous_lerp(...)`（game util，采样注入）；`bilinear_sample
///   at_world_xz<RasterBufferRef<512>>` 水格采样 `<=0` → state=2。
/// - default（行走）：`dy = (terrain_h + OFFSET) - pos.y`；
///   * 地形低于羊：dy clamp[-2,2]，`v += (dy*WALK_GAIN - v)*smooth`；
///   * 地形高于羊：`pos.y = terrain_h`，dy clamp[-5,5]，`v += (dy*WALK_GAIN - v)*smooth*3`；
///   * 写回 `v @ +4`。
/// - 6（浮空）：dy clamp[-0.3,0.3]，`v += (dy*FLOAT_GAIN - v)*smooth`；`dy > -0.05` → state=2、v=0。
///
/// `lerped_y` / `in_water` 由系统注入（time_continuous_lerp / 水格采样为 game 调用）。
/// 返回 (pos.y, 竖向速度, 状态)。
pub fn apply_terrain_follow(
    state: TerrainFollowerState,
    terrain_h: f32,
    pos_y: f32,
    vel: VerticalDelta,
    dt: f32,
    lerped_y: f32,
    in_water: bool,
) -> (f32, VerticalDelta, TerrainFollowerState) {
    let smooth = 1.0 - (-3.0 * dt).exp();
    match state.0 {
        STATE_GROUNDED => (terrain_h, vel, state),
        STATE_TRANSITION => {
            let y = lerped_y;
            if in_water {
                (y, vel, TerrainFollowerState(STATE_GROUNDED))
            } else {
                (y, vel, state)
            }
        }
        STATE_FLOATING => {
            let dy = terrain_h - pos_y;
            let c = dy.clamp(-0.3, 0.3);
            let v = vel.0 + (c * FLOAT_GAIN - vel.0) * smooth;
            if dy > -0.05 {
                (pos_y, VerticalDelta(0.0), TerrainFollowerState(STATE_GROUNDED))
            } else {
                (pos_y, VerticalDelta(v), state)
            }
        }
        _ => {
            let dy = (terrain_h + OFFSET) - pos_y;
            if terrain_h <= pos_y {
                let c = dy.clamp(-2.0, 2.0);
                let v = vel.0 + (c * WALK_GAIN - vel.0) * smooth;
                (pos_y, VerticalDelta(v), state)
            } else {
                let c = dy.clamp(-5.0, 5.0);
                let v = vel.0 + (c * WALK_GAIN - vel.0) * smooth * 3.0;
                (terrain_h, VerticalDelta(v), state)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grounded_snaps_to_height() {
        let (y, v, s) = apply_terrain_follow(TerrainFollowerState(STATE_GROUNDED), 7.0, 3.0, VerticalDelta(0.0), 0.016, 0.0, false);
        assert_eq!(y, 7.0);
        assert_eq!(v, VerticalDelta(0.0));
        assert_eq!(s, TerrainFollowerState(STATE_GROUNDED));
    }

    #[test]
    fn floating_sinks_when_terrain_below() {
        // 浮空且地形远低于羊 -> dy clamp 到 -0.3，速度向下（<0）缓慢接近地形，状态保持 6。
        let (y, v, s) = apply_terrain_follow(TerrainFollowerState(STATE_FLOATING), 0.0, 10.0, VerticalDelta(0.0), 0.016, 0.0, false);
        assert_eq!(s, TerrainFollowerState(STATE_FLOATING));
        assert!(v.0 < 0.0);
        assert_eq!(y, 10.0);
    }
}
