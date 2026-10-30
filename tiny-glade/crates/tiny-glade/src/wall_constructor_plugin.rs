//! wall-constructor CPU 注册（第 11 轮 WC②）。
//!
//! 原 `crates/tiny-glade/src/wall_constructor_plugin.rs` 的恢复入口。真实
//! 系统集合与阶段顺序由 `system_wall_constructor::add_wall_constructor_systems`
//! 提供（`WallCpuStage::Edit/Bricks/Holes`，见该 crate `lib.rs`）。
//!
//! 中央接线（本文件不修改 `run_game.rs`）：
//! 1. 在 `crates/tiny-glade/src/lib.rs` 增加
//!    `pub mod wall_constructor_plugin;`；
//! 2. App 构建处调用 [`build`]（初始化本 crate 的 5 个资源）；
//! 3. 在 `country-core/src/startup/run_game.rs::register_update_systems`
//!    的 `add_wall_cpu_systems(schedule);` 之后插入一行：
//!    `system_wall_constructor::add_wall_constructor_systems(schedule);`
//!    （等价于调用 [`register_cpu_systems`]）。
use bevy_app::App;
use bevy_ecs::schedule::Schedule;

/// 初始化 wall-constructor 自有资源。
///
/// 系统消费的 core 资源（`PublicWalls`/`WallHoles`/`Gates`/`PublicWalls` 事件、
/// `AssetSsboLibrary` 等）由 core 的 `register_wall_cpu_resources` 负责，
/// 这里只登记本 crate 的 5 个 `Resource`。
pub fn build(app: &mut App) {
    app.init_resource::<system_wall_constructor::construct_arches::WallConstructorOutput>();
    app.init_resource::<system_wall_constructor::construct_arches::WallPathSegmentationMasks>();
    app
        .init_resource::<system_wall_constructor::construct_lintels::WallPathSegmentationMasksMinusStairs>();
    app.init_resource::<system_wall_constructor::construct_platform_stairs::PlatformStairsStorage>(
    );
}

/// 将真实 wall-constructor 系统集合注册到墙 CPU 调度。
///
/// `schedule` 必须是 `country-core::startup::run_game::register_update_systems`
/// 正在构建的同一个 `Schedule`；`add_wall_cpu_systems` 已配置
/// `WallCpuStage` 的 chain，本注册直接复用同一组 SystemSet。
pub fn register_cpu_systems(schedule: &mut Schedule) {
    system_wall_constructor::add_wall_constructor_systems(schedule);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_initializes_constructor_resources() {
        let mut app = App::new();
        build(&mut app);
        assert!(
            app.world()
                .get_resource::<system_wall_constructor::construct_arches::WallConstructorOutput>()
                .is_some()
        );
        assert!(
            app.world()
                .get_resource::<system_wall_constructor::construct_lintels::WallPathSegmentationMasksMinusStairs>()
                .is_some()
        );
        assert!(
            app.world()
                .get_resource::<system_wall_constructor::construct_platform_stairs::PlatformStairsStorage>()
                .is_some()
        );
    }
}
