//! 构造工具模块（utils/mod.rs）。
//!
//! `perturb_row_ends`（0x141240d30）/`split_horizontally`（0x141240ec0）：
//! from_curve（0x140769760）行末扰动与横向切分 helper。两个函数均已按
//! 原机器码逐指令重建（RNE 0x141240d30..0x141240eb4、0x141240ec0..
//! 0x141241117；辅助 Aabb2::center/width/height 0x1412400c0/0x1412402e0/
//! 0x1412402f0、from_center_dims 0x14123fff0）。常量 1.99=0x3ffeb852、
//! 0.4=0x3ecccccc、0.3=0x3e99999a、0.5=0x3f000000、1.0=0x3f800000。
pub mod half_timber;
pub mod resolve_hole_overlap;
pub mod sine_wave;
pub mod split_into_random_rows;
pub mod stairs_signifiers;
pub mod trim_rows;
pub mod wall_corners;
pub mod wall_skirts;

use std::collections::VecDeque;

/// 40B 砖记录（原 `MyBrick`：aabb0..15/deformation16..31/seed32..35/
/// split_y36/split_x37/padding38..39）。`deformation` 存切分后的四角坐标
/// （lower={min.x,min.y,new.x,new.y}、upper={new.x,new.y,max.x,max.y}）。
#[derive(Clone, Copy, Debug)]
pub struct MyBrick {
    /// aabb（0..15）：{min_x, min_y, max_x, max_y}。
    pub aabb: [f32; 4],
    /// deformation（16..31）：四角坐标（渲染 quad）。
    pub deformation: [f32; 4],
    /// seed（32..35）。
    pub seed: u32,
    /// split_y（36）。
    pub split_y: u8,
    /// split_x（37）。
    pub split_x: u8,
}

/// B `perturb_row_ends`（0x141240d30）：扰动行末砖的 aabb 端点。
///
/// 机器码 RNE 0x141240d30..0x141240eb4：
/// - 参数：rcx=&mut aabb(4×f32)、rdx=end(0=前向，len-1=后向)、
///   r8/r9=rows(&[f32])、[rsp+0x28]=next(usize)、[rsp+0x30]=amplitude(f32)、
///   [rsp+0x38]=magnitude(f32)。
/// - 入口调 Aabb2::center（0x1412400c0）→ center.x=(aabb[0]+aabb[2])*0.5。
/// - 前向（end==0）：t = 1.0 − 0.5*(rows[next]+rows[next−1])；
///   v = NaN 感知 min(center, t) × 1.99（0x3ffeb852）；再 min(amplitude, v)；
///   写回 aabb[0] = v*magnitude + aabb[0]。
/// - 后向（end==len−1，next−1 必须等于 end，否则直接返回）：
///   mid = 0.5*(rows[0]+rows[1])；v = NaN 感知 min(1.0−center, mid) × 1.99；
///   再 min(amplitude, v)；写回 aabb[2] = v*magnitude + aabb[2]（rsi+8）。
/// - 索引 rows[next]、rows[next−1]、rows[0]、rows[1] 触发 panic_bounds_check。
pub fn perturb_row_ends(
    aabb: &mut [f32; 4],
    end: usize,
    rows: &[f32],
    next: usize,
    amplitude: f32,
    magnitude: f32,
) {
    const MAG: f32 = f32::from_bits(0x3ffeb852); // 1.99
    let center = (aabb[0] + aabb[2]) * 0.5;
    if end == 0 {
        let t = 1.0 - (rows[next] + rows[next - 1]) * 0.5;
        let v = f32::min(center, t);
        let v = f32::min(amplitude, v * MAG);
        aabb[0] = v * magnitude + aabb[0];
    } else {
        if next - 1 != end {
            return;
        }
        let mid = (rows[0] + rows[1]) * 0.5;
        let t = 1.0 - center;
        let v = f32::min(mid, t);
        let v = f32::min(amplitude, v * MAG);
        aabb[2] = v * magnitude + aabb[2];
    }
}

/// B `split_horizontally`（0x141240ec0）：按 t = split*0.4+0.3 切分砖，
/// 推两块 MyBrick 到 VecDeque。
///
/// 机器码 RNE 0x141240ec0..0x141241117：
/// - 参数：xmm0=split(f32)、rdx/r8=同一 aabb(4×f32)、r9d=seed(u32)、
///   [rsp+0x28]=out(&mut VecDeque<MyBrick>)。
/// - t = split×0.4（0x3ecccccc）+0.3（0x3e99999a）。
/// - 高度 h=max_y−min_y、宽 w=max_x−min_x、center.x=(min_x+max_x)*0.5；
///   new = lerp(min, max, t)（两轴，movsldup/mulps/addps）。
/// - lower：aabb={min_x, min_y, max_x, new_y}（from_center_dims
///   center={center.x, min_y+h*t*0.5} dims={w, h*t}）；deformation=
///   {min_x, min_y, new_x, new_y}；seed=原值；split=0。
/// - upper：aabb={min_x, new_y, max_x, max_y}（center={center.x,
///   max_y−h*(1−t)*0.5} dims={w, h*(1−t)}）；deformation=
///   {new_x, new_y, max_x, max_y}；seed=原值+1；split=0。
pub fn split_horizontally(
    split: f32,
    aabb: &[f32; 4],
    seed: u32,
    out: &mut VecDeque<MyBrick>,
) {
    const SPLIT_A: f32 = f32::from_bits(0x3ecccccc); // 0.4
    const SPLIT_B: f32 = f32::from_bits(0x3e99999a); // 0.3
    let t = split * SPLIT_A + SPLIT_B;
    let (min_x, min_y, max_x, max_y) = (aabb[0], aabb[1], aabb[2], aabb[3]);
    let h = max_y - min_y;
    let w = max_x - min_x;
    // 原机器码在此经 Aabb2::center 计算 center.x，再由 from_center_dims
    // （center={center.x, lower/upper 中心 y}，dims={w, h*t | h*(1-t)}）
    // 还原 x 区间恒为 [min_x, max_x]、y 区间按 t 切分。
    let new_x = min_x + t * w; // (1-t)*min_x + t*max_x
    let new_y = min_y + t * h; // (1-t)*min_y + t*max_y
    // lower
    out.push_back(MyBrick {
        aabb: [min_x, min_y, max_x, new_y],
        deformation: [min_x, min_y, new_x, new_y],
        seed,
        split_y: 0,
        split_x: 0,
    });
    // upper
    out.push_back(MyBrick {
        aabb: [min_x, new_y, max_x, max_y],
        deformation: [new_x, new_y, max_x, max_y],
        seed: seed.wrapping_add(1),
        split_y: 0,
        split_x: 0,
    });
}
