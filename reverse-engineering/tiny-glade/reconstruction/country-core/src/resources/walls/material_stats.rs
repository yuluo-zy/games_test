//! material_stats.rs — 墙材质统计（WP12 系统向 facade）。
//!
//! O 名；布局 U（typed gap）。

#![allow(dead_code, unused_imports)]

use crate::typed_gap::Gap;

#[derive(Clone, Debug, Default)]
pub struct MaterialStats {
    pub raw: Gap,
}
