//! walls/mod.rs — 墙资源族根（WP11 状态类型）。
//!
//! 源骨架 `resources/walls/mod.rs`（B）；PDB 命名实体：`Floor` / `HighlightColor` /
//! `WallHighlight` / `WallId`。本文件只含 WP11 已恢复的状态类型；骨架中
//! 匿名 `_::impl$0` serde（WallId JSON 数字串）与 `impl$17::register_required_components`
//! 归属集成组装。

#![allow(dead_code, unused_imports)]

use bevy_ecs::component::Component;
use glam::Vec3;

use crate::typed_gap::Gap;

/// 全局墙 ID（O：u64 newtype）。
///
/// 证据：`WallId::get_inner` 0x140885d0 返回内值；`WallId::is_valid` 0x140b34f20
/// 机器码 `cmp/test`+`setne` 测 `u64 != 0`（先验 decorator-info：`WallId::is_valid`
/// 140814277 调用 140b34f20 判定 platform 有效）；`_::impl$0::serialize`
/// 0x140b35980 十进制写 Vec<u8>（JSON 数字串）。Hashbrown HashMap 键（O，PDB）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct WallId(pub u64);

impl WallId {
    /// O：0x140885d0 返回内部 u64。
    #[inline]
    pub fn get_inner(self) -> u64 {
        self.0
    }
    /// O：0x140b34f20 机器码 `u64 != 0` 有效。
    #[inline]
    pub fn is_valid(self) -> bool {
        self.0 != 0
    }
}

/// 楼板组件（O：ECS `Option<&Floor>`；`nani_update_stone_floor` 读 +8 字节 bit0）。
/// 布局 I：≥0x28 字节；+0x8 bit0 控制屋檐切线路径（WP6 登记）。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct Floor(pub Gap);

/// 墙高亮组件（O：PDB 命名实体；render 高亮消费）。布局 U。
#[derive(Component, Clone, Copy, Debug, Default)]
pub struct WallHighlight(pub Gap);

/// 高亮色枚举（O：`to_rgb` 0x140b34f30 字节判别两分支；变体名 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum HighlightColor {
    /// 判别字节 0 → RGB(0.07, 0.08, 0.07)（O 机器码 xmm 常量）。
    #[default]
    V0,
    /// 判别字节 1 → RGB(0.1, 0.03, 0.0)（O 机器码）。
    V1,
}

impl HighlightColor {
    /// O：0x140b34f30。判别字节 ==1 → (0.1,0.03,0)；否则 (0.07,0.08,0.07)。
    #[inline]
    pub fn to_rgb(self) -> Vec3 {
        match self {
            HighlightColor::V0 => Vec3::new(0.07, 0.08, 0.07),
            HighlightColor::V1 => Vec3::new(0.1, 0.03, 0.0),
        }
    }
}

/// O：0x140b34f80。墙"裙边"横带高度。
///
/// 机器码：`height >= 5.9` 才产生边界；首边界 4.5，随后 +3.5 递增，
/// 循环条件 `height > f + 1.4`（即 `height <= f+1.4` 停止）。
/// 返回空 Vec（height < 5.9）。
pub fn split_height_into_skirts(height: f32) -> Vec<f32> {
    if height >= 5.9 {
        let mut skirts = vec![4.5f32];
        let mut f = 4.5f32 + 3.5;
        while height > f + 1.4 {
            skirts.push(f);
            f += 3.5;
        }
        skirts
    } else {
        Vec::new()
    }
}
