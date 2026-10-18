//! B `crates/tiny-glade/src/app_systems.rs`（墙系统/事件注册 + glade 卸载清理）。
//!
//! 恢复依据（Ghidra shared 2026-10-17）：
//! - `impl$0::add_wall_systems` `0x140493120`（1,131 B，13 KB 反编译）
//! - `impl$0::add_wall_events` `0x1404936b0`（5,473 B，39 KB 反编译）
//! - `walls_on_glade_unload` `0x1402b1cb0`（3,765 B，15 KB 反编译）
//!
//! ## `add_wall_systems` 已解出的注册顺序（O，泛型签名 = PDB 真实类型）
//! 1. `Update`：`ui_arch_controls(
//!    Res<AppMode>, Res<PublicWalls>, Res<ArchSegments>, Res<UiState>,
//!    EventWriter<DoUiArchControls>)` `.in_set(GameSystemSet)`
//! 2. `Update`：`(Res<AppMode>, Res<InnerWalls>,
//!    ResMut<DecoratorArchivist>, ResMut<DecoratorStorage>,
//!    ResMut<DecoratorPhysicsColliders>, ResMut<RaycastWorld>,
//!    ResMut<StairsState>, EventReader<ModeJustChanged>)` `.in_set(GameSystemSet)`
//! 3. `Update`：一组 `.run_if(...)` + 两次 `chain_ignore_deferred` +
//!    `in_set(...)`（系统名/条件体需逐个反编译锚定）
//! 4. `insert_resource(OverhangSupportDirtySubscriptions)`
//! 5. `Startup`：`detect_overhang_support_segments` 族
//!    `(ResMut<OverhangSupportDirtySubscriptions>, ResMut<RaycastWorld>,
//!    ResMut<TerrainHeightsData<TerrainHeightSsbo>>)`，链式 + `in_set`
//! 6. `Update`：渲染上传组
//!    `(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>,
//!    Query<&mut InstancedWall>)` 与
//!    `(Res<GraphicsDeviceResource>, ResMut<AssetSsboLibrary>)`
//!    `.in_set(GameSystemSet)`
//! 7. `PostUpdate`：`(EventReader<ShutdownGameCmd>, Commands)` 与
//!    `closure(Res<State<AppLoadingState>>, ResMut<NextState<AppLoadingState>>)`
//!    （= `walls_on_glade_unload` + 状态切换）
//!
//! 未实现原因：第 2/3/5/6 组的具体系统函数名与条件闭包未逐个锚定；
//! `DecoratorPhysicsColliders`、`DecoratorStorage`、`RaycastWorld` 的 ResMut
//! 面在 country-core 存在但系统函数体/签名不完全一致；`Missing` 任一都会
//! 造成虚假接线。`add_wall_events`（39 KB）为事件注册表，尚未逐项解出。
//!
//! `walls_on_glade_unload` 属真实业务（卸载时清理墙/装饰物/碰撞），
//! 15 KB 反编译，尚未逐语句重建。

/// `tiny_glade::app_systems::impl$0::add_wall_systems` `0x140493120` —— 未实现。
pub fn add_wall_systems() {
    todo!(
        "未实现: add_wall_systems 0x140493120 —— 注册顺序已解（见模块注释），\
         系统函数名/ResMut 面未逐个锚定"
    );
}

/// `tiny_glade::app_systems::impl$0::add_wall_events` `0x1404936b0` —— 未实现。
pub fn add_wall_events() {
    todo!("未实现: add_wall_events 0x1404936b0 —— 39KB 事件注册表未逐项解出")
}

/// `tiny_glade::app_systems::walls_on_glade_unload` `0x1402b1cb0` —— 未实现。
pub fn walls_on_glade_unload() {
    todo!("未实现: walls_on_glade_unload 0x1402b1cb0 —— 15KB 卸载清理业务，未逐语句重建")
}
