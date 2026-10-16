//! B `cottage_dormers/collision_geo.rs` — 老虎窗碰撞几何（D1）。
//!
//! 证据：fresh RE-MCP 反编译 0x14128a0b0（get_collision_mesh，50KB）+ 骨架
//! callees（O）：DormerKey::quantized_from（内联）、inverse_lerp、swizzlers
//! xvy ×7、递归 get_collision_mesh（自调用 1 次）、FUN_141289c80 ×2（子网格
//! 构建）。
//!
//! 业务（O 结构）：DormerKey（量化键）→ 碰撞网格构建（递归细分，inverse_lerp
//! + swizzler 消费）。U：网格数学主体。
//! U 登记：① DormerKey 量化准则 ② 网格细分算法。

use glam::Vec3;

/// DormerKey（O 类型名；I：{子类型, 变体} 量化键，语义 U）。
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct DormerKey {
    pub subtype: u8,
    pub variant: u8,
}

impl DormerKey {
    /// DormerKey::quantized_from（内联，I；U：量化准则）。
    pub fn quantized_from(_info: &country_core::resources::walls::decorator::DecoratorInfo) -> Self {
        Self { subtype: 0, variant: 0 } // U。
    }
}

/// 0x14128a0b0（O 结构）：老虎窗碰撞网格（递归细分构建）。
/// U：网格顶点/面数学主体。
pub fn get_collision_mesh(_key: DormerKey) -> Vec<Vec3> {
    Vec::new() // U。
}
