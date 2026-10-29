//! system_water — 水面编辑系统族（组 A restorer，2026-10-16；fresh 重建 2026-10-17）。
//!
//! 原 crate（B skeleton）：`crates/systems/water/`（4 文件；16 符号 = 业务 10 /
//! 内联 1 / 胶水 5；本 crate 只承载独立业务过程，zeno 7 项与 `TwoWaySearcher`
//! 判 dependency_reuse 不恢复）。
//!
//! - `draw_water`（0x141ba9130）：水/水面橡皮双模式绘制（AppMode::Water /
//!   AppMode::Eraser(All|Water)），经 `world_raster::draw_raster` 写 WaterRaster，
//!   发射 `WaterDrawEvent`/`WaterEraseEvent`（core `resources::water`）。fresh：
//!   橡皮分支 `clearing_canvas("eraser",6,0)` + 光标交点指示符 + draw_raster 点 =
//!   交点；水分支 `clearing_canvas("watereraser_water_",5,4)` + 界内圈 + 图钉（case2）。
//! - `update_water_audio_grid`（0x141baefe0）：相机周围 64 帧滚动活动位集 × 抖动
//!   增量计数器，填充 core `WaterAudioGrid`（语义投影）。fresh：采样点含每帧抖动
//!   `d*0.0625*13.0`；计数器按增量对 ±d（非 ±1，core 叶子 `apply_audio_bit` 已修正为
//!   原 PADDW/PSUBW 语义）。
//! - `record_water_history`（行 12，PDB inlinee）：`RecordWaterEditToHistoryCmd`
//!   `read().last()` 未读 → `record_raster_history`（槽位 4 / `EditType::Water`）。
//!   fresh：无 `HistoryRecordingFailed` 发射（删除先前 garden 惯例近似）。
//!
//! 共享门禁：进程全局 `*0x14362c098`（非 0 时水面编辑/音频格更新早退；render 域
//! 章节记为 demo 开关，re-analyst 记为输入/UI 抢占——两种语义都要求"非 0 跳过"，
//! 本 crate 以 [`water_systems_enabled`] 可覆写常量建模）。

pub mod draw_water;
pub mod record_water_history;
pub mod water_audio_grid;

use bevy_ecs::prelude::*;

use country_core::resources::world_raster::DirtyTilePublisher;
use country_core::resources::world_raster_defs::WaterRaster;

use crate::draw_water::draw_water;
use crate::record_water_history::{record_water_history, RecordWaterEditToHistoryCmd};
use crate::water_audio_grid::update_water_audio_grid;

/// 进程全局门禁地址（O：`cmp byte ptr [0x14362c098], 0`；非 0 → 早退）。
/// 写入者 = `tiny_glade::main`（0x1403dcf78，值 0，round3 O）。
pub const WATER_GATE_FLAG_ADDR: u64 = 0x1_4362_c098;

/// 门禁建模（I）：重建库不可直读宿主内存。`WATER_GATE_FLAG` 为假 = 宿主标志为 0
/// （正常编辑）；宿主接入真实全局时可改为按字节读。
pub fn water_systems_enabled() -> bool {
    WATER_GATE_FLAG
}
const WATER_GATE_FLAG: bool = true;

/// 本地可运行顺序（O 结构：`build` 0x1406501d0 有 2 次 `add_systems` + 1 次
/// `in_set_inner`；标签行 16/34。精确集合名 U，按 core garden 域惯例落地
/// Draw → Record 链）。
#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub enum WaterCpuStage {
    Draw,
    Record,
}

/// 事件与资源注册（O：`build` 0x1406501d0，glue-legitimate：`register_event`×3 =
/// `RecordWaterEditToHistoryCmd` / `WaterDrawEvent` / `WaterEraseEvent`（各带
/// "已存在则跳过"判空）；`insert_resource<WaterRaster>`（`DirtyTilePublisher::unique()`
/// 0x140e93590 + `PathRaster::new()` 0x14083eaf0，单态内联进构造；插槽仅 WaterRaster）。
///
/// 本 crate 追加注册自身系统写出的其余事件（core 全游戏由其它插件注册）——
/// crate 独立可运行的自足接线，非原 build 面（差登记 RESTORE.md）。
pub fn register_water_events(world: &mut World) {
    use country_core::resources::glade_border::{DisplayOutOfBorder, FeedbackInputOutOfBorder};
    use country_core::resources::onboarding::NotifyHintSystemToolWasJustUsed;
    use country_core::resources::water::{WaterDrawEvent, WaterEraseEvent};
    use country_core::systems::history::history_process::DebugHistoryMsg;
    use country_core::systems::render_loop::compute_passes::rasterize_terrain_stroke::RecalculateHeightMapCmd;
    use country_core::systems::terrain::delayed_terrain_changed_msg::OnTerrainChange;

    // O：insert_resource<WaterRaster>（`WaterRaster::new(DirtyTilePublisher::unique())`）。
    world.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
    world.init_resource::<Events<RecordWaterEditToHistoryCmd>>();
    world.init_resource::<Events<WaterDrawEvent>>();
    world.init_resource::<Events<WaterEraseEvent>>();
    world.init_resource::<Events<RecalculateHeightMapCmd>>();
    world.init_resource::<Events<DisplayOutOfBorder>>();
    world.init_resource::<Events<FeedbackInputOutOfBorder>>();
    world.init_resource::<Events<NotifyHintSystemToolWasJustUsed>>();
    world.init_resource::<Events<DebugHistoryMsg>>();
    world.init_resource::<Events<OnTerrainChange>>();
    world.init_resource::<Events<draw_water::WaterPinnedClutterCircle>>();
}

/// 注册 CPU 水面系统（O：build 0x1406501d0 的 add_systems 面；集合名 I）。
///
/// 原 build 只 add **两个**系统：`record_water_history.in_set(GameSystemSet)` 与
/// `update_water_audio_grid.run_if(Option<Res<WaterAudioGrid>>.is_some()).in_set(<dyn set>)`
/// ——`draw_water` 不由本插件注册（其注册在别的插件/集成层，模块外接线）。本 crate 为
/// 独立库自足运行而追加 `draw_water`（差登记 RESTORE.md）。
pub fn add_water_cpu_systems<C: Send + Sync + 'static, S: Send + Sync + 'static>(
    schedule: &mut Schedule,
) {
    use country_core::systems::history::HistoryCpuStage;
    schedule.configure_sets((WaterCpuStage::Draw, WaterCpuStage::Record).chain());
    schedule.configure_sets(
        WaterCpuStage::Draw
            .after(HistoryCpuStage::Pop)
            .before(HistoryCpuStage::Record),
    );
    schedule.add_systems(draw_water::<C>.in_set(WaterCpuStage::Draw));
    schedule.add_systems(update_water_audio_grid.in_set(WaterCpuStage::Draw));
    schedule.add_systems(record_water_history::<S>.in_set(WaterCpuStage::Record));
}

/// 仅供本 crate 独立运行/测试的 `(world, schedule)` 落地（旧约定，集合名为本地发明的
/// [`WaterCpuStage`]）。真实游戏经 [`WaterPlugin`] 注册，二者不要同时使用。
pub fn build<C: Send + Sync + 'static, S: Send + Sync + 'static>(
    world: &mut World,
    schedule: &mut Schedule,
) {
    register_water_events(world);
    add_water_cpu_systems::<C, S>(schedule);
}

/// `<system_water::WaterPlugin as Plugin>::build`（0x1406501d0）。
///
/// 机器码（PyGhidra 逐指令 + `Box<dyn System>` vtable/类型名串核对）：
/// 1. `add_systems(Update, (draw_water.in_set(0x10), record_water_history.in_set(0x22)))`
///    ——同一次 tuple 注册，各系统只属于一个 set（in_set 单体 0x140e92740 / 0x140e930c0，
///    类型名串 0x142b8b67d / 0x142b8bb05）；
/// 2. `add_systems(Update, update_water_audio_grid.run_if(Option<Res<WaterAudioGrid>>.is_some())
///    .in_set(0x24))`——独立注册，只在 0x24（`in_set_inner`，名串 0x142b8c1c0）；
/// 3. `insert_resource<WaterRaster>`（无条件，`DirtyTilePublisher::unique()` + `new`）；
/// 4. 三次 `register_event`（各带 TypeId 存在性检查）：`RecordWaterEditToHistoryCmd`、
///    `WaterDrawEvent`、`WaterEraseEvent`。
///
/// 旧文档“`draw_water` 不由本插件注册”来自早期反编译，被机器码纠正：它在第 1 条 tuple 里。
/// `draw_water<C>` 的 `C` 取原版 native-physics 构建的碰撞载荷，`record_water_history<S>` 的 `S`
/// 为 `MaybePendingSnapshotId`。
pub struct WaterPlugin;

impl bevy_app::Plugin for WaterPlugin {
    fn build(&self, app: &mut bevy_app::App) {
        use bevy_app::Update;
        use country_core::resources::history::MaybePendingSnapshotId;
        use country_core::resources::walls::decorator_archivist::LiveColliderPayload;
        use country_core::resources::water::{WaterDrawEvent, WaterEraseEvent};
        use country_core::systems::GameSystemSet;
        use country_core::systems::audio::water_ambience_audio::WaterAudioGrid;

        app.add_systems(
            Update,
            (
                draw_water::<LiveColliderPayload>.in_set(GameSystemSet(0x10)),
                record_water_history::<MaybePendingSnapshotId>.in_set(GameSystemSet(0x22)),
            ),
        );
        app.add_systems(
            Update,
            update_water_audio_grid
                .run_if(|grid: Option<Res<WaterAudioGrid>>| grid.is_some())
                .in_set(GameSystemSet(0x24)),
        );
        app.insert_resource(WaterRaster::new(DirtyTilePublisher::unique()));
        app.add_event::<RecordWaterEditToHistoryCmd>();
        app.add_event::<WaterDrawEvent>();
        app.add_event::<WaterEraseEvent>();
        // 与ClutterPlugin幂等注册同一ShowPinned事件，没有水工具专属替身队列。
        app.add_event::<draw_water::WaterPinnedClutterCircle>();
    }
}
