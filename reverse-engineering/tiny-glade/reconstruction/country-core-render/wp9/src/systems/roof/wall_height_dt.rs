//! wall_height_dt.rs — wall_height_dt 系统（WP9）。
//!
//! PDB 骨架 `systems/roof/src/wall_height_dt.rs`（line 4）。机器码 0x1421a2b60。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::Roof;
use crate::typed_gap::Gap;

/// wall_height_dt（O：0x1421a2b60）。
///
/// 每屋顶 tile 索引（Roof stride 0x58）：`delta = Roof[0x4c] - wall_base_y`，写 tile 记录
/// `[+0]=delta, [+4]=Roof[0x4c]`（stride 0x18），另写 u32 tick（stride 4）。
/// → gravity push `wall_height_delta` 的数据源。
pub fn wall_height_dt(
    _roofs: &[Roof],
    _wall_y: Gap,
    _delta_out: &mut [f32],
    _tick_out: &mut [u32],
) {
    for (i, r) in _roofs.iter().enumerate() {
        // 占位：wall_base_y 关联 U。
        if let Some(d) = _delta_out.get_mut(i * 2) {
            *d = r.base_y();
        }
        if let Some(y) = _delta_out.get_mut(i * 2 + 1) {
            *y = r.base_y();
        }
        if let Some(t) = _tick_out.get_mut(i) {
            *t = 0;
        }
    }
}
