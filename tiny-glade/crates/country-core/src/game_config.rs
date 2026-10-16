//! game_config.rs — `country_core::game_config` 游戏配置资源（组 A restorer 接线）。
//!
//! 源（B skeleton）：`crates/country-core/src/game_config.rs`；章节
//! `chapters/game_config.rs.md`（业务 4 / 胶水 10，全部机器码手解）：
//! - `GameConfig::upgrade` @0x14090df90：`test byte[rcx+0x88],1; je +1; ret`（+0x88
//!   位 0 已置即返回）；`[rcx+0x84]` f32 比较 1.0@0x142925904 与 0.125@0x142aab384；
//!   `cmp dword[rcx+0x80],0`、写 `[rcx+0x80]=0`、`[rcx+0x88]=1`、`[rcx+0x8c]=旧值`
//!   —— 配置版本迁移（字段语义 U）。
//! - `DevSettings::live_link` @0x14090df40：`movzx eax,[rcx+7]`（dev 设置字节 @+0x07）。
//! - `developer_hotkeys_enabled` @0x14090e070 / `is_within_glade_shape` @0x14090dfe0：
//!   共用进程全局字节 0x14362d6dd（开发者模式开关）。
//! - `system_glade_border::display_glade_border` 0x141be0ad0 读取 `+0x121` 字节
//!   （== 0 时写边界 signifier；语义 U）。
//!
//! 本实现为语义结构（非字节对齐，按会话域惯例）；配置解析/serde 属派生胶水不计。

use bevy_ecs::resource::Resource;

use crate::typed_gap::Gap;

/// 游戏配置（O 面：upgrade 迁移字段 @+0x80/@+0x84/@+0x88/@+0x8c；边界 signifier
/// 开关字节 @+0x121；其余 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct GameConfig {
    /// O：`display_glade_border` 0x141be0ad0 读取 `+0x121`；== 0 时写边界
    /// signifier（语义 U，字段名 I）。
    pub border_signifier_enabled: bool,
    /// typed gap：upgrade 迁移字段（@+0x80 u32 / @+0x84 f32 / @+0x88 u64 位 0 /
    /// @+0x8c 旧值）与其余配置。
    pub _gap: Gap,
}

impl GameConfig {
    /// `GameConfig::upgrade` @0x14090df90（O：版本迁移；+0x88 位 0 已置即返回）。
    /// 语义模型：本实现无迁移字段实体，登记机器码行为（字段语义 U），不造数据。
    pub fn upgrade(&mut self) {}
}

/// dev 设置（O 面：`live_link` 读字节 @+0x07；其余 U）。
#[derive(Resource, Clone, Debug, Default)]
pub struct DevSettings {
    /// O：+0x07 字节（`DevSettings::live_link` @0x14090df40 读取）。
    pub live_link: bool,
    /// typed gap。
    pub _gap: Gap,
}

/// `developer_hotkeys_enabled` @0x14090e070（O：读全局字节 0x14362d6dd）。
/// 与 `is_within_glade_shape`（0x14090dfe0，椭圆边界测试启用位）共用。
pub const DEVELOPER_MODE_FLAG_ADDR: u64 = 0x1_4362_d6dd;

/// 开发者模式建模（I：宿主按全局字节接线；真值 = 开发者热键/glade 椭圆测试启用）。
pub fn developer_hotkeys_enabled() -> bool {
    DEVELOPER_MODE
}
const DEVELOPER_MODE: bool = false;
