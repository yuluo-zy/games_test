//! simulate_roof_tile_for_audio.rs — 屋瓦音频模拟系统（WP9）。
//!
//! PDB 骨架 `systems/roof/src/simulate_roof_tile_for_audio.rs`（`simulate_roof_tile` inline、
//! `simulate_roof_tile_for_audio`）。机器码 0x1421a7090。

#![allow(dead_code, unused_imports)]

use crate::resources::roofs::roof_shape::Roof;
use crate::typed_gap::Gap;

/// 单瓦音频模拟记录（stride 0x18=24B）。
/// O：`[+0]=height、[+3]=velocity、[+4]=sound_timer、[+5]=active byte`；[+2] 语义 U。
#[derive(Clone, Copy, Debug, Default)]
pub struct AudioTileSim {
    pub height: f32,
    pub pivot: f32,
    pub vel: f32,
    pub velocity: f32,
    pub sound_timer: f32,
    pub active: u8,
}

/// simulate_roof_tile_for_audio（O：0x1421a7090）。
///
/// 1. 时间资源 dt（+0x28）为 0 → 早退；跳过 flat 屋顶（`Roof::is_flat` 循环）。
/// 2. 每 tile：`v' = v + dt·(-45.126003)`；`h' = h + dt·v'`（含 clamp）；
///    `h_eased = lerp(h', pivot_y, 1-exp(dt·-13.3))`。
/// 3. 落地（`|h_eased-pivot|<0.1`）：timer 复位、active=0。
///    从 0.1 开始下落：发音频/装饰事件（变体 4=音量 lerp、5=命中）。
///    下落中 `v<0`：`timer += dt`，active=1。
/// 数值常量（O）：-45.126003、15.0、0.166、-13.3、0.1、0.01/0.19。
pub fn simulate_roof_tile_for_audio(
    _dt: f32,
    _roofs: &[Roof],
    _tiles: &mut [AudioTileSim],
    _audio_events: Gap,
) {
    if _dt == 0.0 {
        return;
    }
    for t in _tiles.iter_mut() {
        if t.active == 0 {
            continue;
        }
        t.velocity += _dt * -45.126003;
        let next = t.height + _dt * t.velocity;
        t.height = if next <= t.pivot { t.pivot.max(next) } else { next };
        let ease = 1.0 - (-_dt * 13.300004).exp();
        t.height = ease * t.pivot + (1.0 - ease) * t.height;
        if (t.height - t.pivot).abs() < 0.1 {
            t.sound_timer = 0.0;
            t.active = 0;
        }
    }
}
