//! Observed zero-sized rendering component tags (B components/mod.rs).
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct SemiTransparent;
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default)]
pub struct FlatLighting;

/// 背景地形标记（O：TypeId `0x66476a13bbcc5e41`，`components::impl$66`；`contains` 判断，存在 → 绑磁盘键 0xbd39551337285fb6）。
/// 真名由 `bevy_ecs::bundle::impl$24::component_ids<KeepOnLevelUnload, DrawableMeshBundle, BackgroundTerrain,
/// DontReceiveContactShadows, OverwriteRoughness, …>`（0x1416dcd10）的 bundle 顺序反查（`render-gap/DIGEST.md §3.6`）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct BackgroundTerrain;
/// 不接收接触阴影（O：TypeId `0xa6c0fc01ec67ef4c`，`components::impl$25`；`Option<&T>` 读 → `bool(!has)`）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DontReceiveContactShadows;
/// 覆盖粗糙度（O：TypeId `0x192600af2434483b`，`components::impl$29`；`f32 @+0`，缺省 0.8）。
#[derive(bevy_ecs::component::Component, Clone, Copy, Debug, Default, PartialEq)]
pub struct OverwriteRoughness(pub f32);
