//! `resources::critters` — 昆虫/动物 GPU SSBO 载体（O 名，字段 typed gap）。
//!
//! 原符号：`country_core::systems::render_loop::resources::critters::
//! {ButterflySsbo, FireflySsbo, FireflyVisibilitySsbo}`（instanced_critters
//! 反编译 read_storage_buffer 实例）。共享 `country-core` 尚未恢复；本束承载
//! typed-gap 版本，集成时并入共享 crate。

use country_core::typed_gap::Gap;

/// 蝴蝶实例 SSBO（O：`read_storage_buffer<ButterflySsbo>`）。
#[derive(Clone, Debug, Default)]
pub struct ButterflySsbo(pub Gap);

/// 萤火虫实例 SSBO（O：`read_storage_buffer<FireflySsbo>`）。
#[derive(Clone, Debug, Default)]
pub struct FireflySsbo(pub Gap);

/// 萤火虫可见性查询 SSBO（O：`read_storage_buffer<FireflyVisibilitySsbo>`；
/// 对应用户 `_critters_firefly_vis_query.cs.hlsl` 可见性查询 compute）。
#[derive(Clone, Debug, Default)]
pub struct FireflyVisibilitySsbo(pub Gap);
