//! `resources` — WP18 本束缺失的 typed-gap 载体。
//!
//! 已恢复的资源类型直接引用共享 `country-core` crate（如
//! `country_core::resources::rocky_terrain`）；本模块只承载共享 crate
//! 尚未恢复、而 draw_shaders 域需要引用的类型（O 名来自反编译符号，字段 U）。

pub mod color_id_roof;
pub mod critters;
pub mod texture_presets;
