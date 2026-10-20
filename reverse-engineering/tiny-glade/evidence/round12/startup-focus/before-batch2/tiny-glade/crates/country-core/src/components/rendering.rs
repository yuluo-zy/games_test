//! 渲染查询使用的标记和逐对象参数。
//!
//! 原作这些组件的 ECS 克隆策略均为 Default，因此不派生 Clone/Copy。

use bevy_ecs::component::Component;

/// B FollowMouse: true follows the camera ray at 45 units; false follows
/// the actual terrain hit (1408efbaf/1408efcff).
#[derive(Component, Debug)]
pub struct FollowMouse(pub bool);

/// 使对象进入半透明绘制路径。
#[derive(Component, Debug, Default)]
pub struct SemiTransparent;

/// 使用平面光照。
#[derive(Component, Debug, Default)]
pub struct FlatLighting;

/// 虚线参数。DebugLine 写入长度 × 4，边界线写入周长 × 0.7。
///
/// B 注册入口 0x140849b70，载荷为一个 f32。
#[derive(Component, Debug, Default, PartialEq)]
pub struct DashedLineLength(pub f32);

/// 背景地形；绘制端按该标记选择背景纹理。
#[derive(Component, Debug, Default, PartialEq, Eq)]
pub struct BackgroundTerrain;

/// 禁止对象接收接触阴影。
#[derive(Component, Debug, Default, PartialEq, Eq)]
pub struct DontReceiveContactShadows;

/// 对象的粗糙度覆盖值；未挂此组件时由绘制端使用 0.8。
///
/// Default 的零值仅用于显式构造，不等于缺少组件时的回退值。
#[derive(Component, Debug, Default, PartialEq)]
pub struct OverwriteRoughness(pub f32);

/// Original B components::StairsStep. startup_terrain writes flags0 for step
/// geometry (@14092732a) and flags1 for stairs pebbles (@1409275e2).
/// Default Bevy clone behavior is retained by not deriving Clone/Copy.
#[derive(Component, Debug, PartialEq, Eq)]
pub struct StairsStep {
    pub flags: u8,
}
