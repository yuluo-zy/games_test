//! `systems::render_loop::resources::critters` — 小动物 GPU SSBO（B 骨架同名文件）。
//!
//! 反编译（O）：
//!   - `fireflies_send_to_gpu` 0x142316760：`Vec<FireflySsbo>` 96 实例，`FrozenMap::get<SsboId,
//!     Box<BufferState>>` + 尺寸 memcmp(0x44) + `cpu_write<FireflySsbo>`。
//!   - `butterflies_send_to_gpu` 0x141d87140：`Vec<ButterflySsbo>` 24 实例，memcmp(0x46)。
//! 实例布局 U（typed gap）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

/// 萤火虫实例 SSBO（96 实例；实例布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FireflySsbo(pub Gap);

/// 蝴蝶实例 SSBO（24 实例；实例布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct ButterflySsbo(pub Gap);

/// 萤火虫可见性查询 SSBO（`_critters_firefly_vis_query` 读，O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct FireflyVisibilitySsbo(pub Gap);

/// 绵羊动画 SSBO（O 名；布局 U）。
#[derive(Clone, Copy, Debug, Default)]
pub struct SheepAnimationSsbo(pub Gap);
