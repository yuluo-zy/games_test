# resources/walls/plaster.rs

> 文件作用：石膏墙组件（Plaster）与剥离动画资源（AnimatedPlasterPeel）+ Recalculate* 命令族；渲染域消费（WP6），无独立业务函数。

函数总计 4：业务 3 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `bevy_ecs::system::schedule_system::impl$1::is_exclusive<bevy_ecs::system::function_system::FunctionSystem<void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>),void (*)(bevy_ecs::event::reader::EventReader<country_core::systems::timelapse_replay::InterruptTimelapse>,bevy_ecs::event::writer::EventWriter<country_core::systems::timelapse_replay::TimelapseReplayFinished>,bevy_ecs::change_detection::ResMut<country_core::systems::timelapse_replay::ReplaySpeedMultiplierOverride>,bevy_ecs::change_detection::ResMut<enum2$<country_core::systems::timelapse_replay::TimelapseState> >,bevy_ecs::change_detection::ResMut<country_core::meta_commander::MetaCommander>,bevy_ecs::change_detection::Res<country_core::resources::session::ActiveSession>)> >` | 0x140002370 | 32 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::plaster::impl$4::clone_behavior` | 0x140002370 | 32 | edit_arrow clone_behavior（xor eax,eax 返回 0） | O | audio-misc\decomp\decomp-0x140002370.txt, camera-time\decomp\decomp-0x140002370.txt, collision-eco\decomp\decomp-0x140002370.txt |
| `country_core::resources::walls::plaster::impl$4::register_required_components` | 0x140990b00 | 32 | 构造/注册/生成 | I | wall\decomp\decomp-0x140990b00.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`default` 等 1 项
## 实现方式（算法级）

### 该文件无独立业务函数（依据：sysres-procedures.json 含 plaster.rs 但仅 3 条 bevy 胶水）

sysres-procedures.json 对 `resources/walls/plaster.rs` 仅登记：
- `bevy_ecs::system::schedule_system::impl$1::is_exclusive<FunctionSystem<…timelapse_replay…>>` 0x140002370（**Bevy 调度胶水，非 plaster 业务**，与其他域同名同址复用）
- `impl$4::clone_behavior` 0x140002370（同址，derive 胶水：`xor eax,eax` 返回 0）
- `impl$4::register_required_components` 0x140990b00（组件注册胶水，I）

即：Plaster 组件 / AnimatedPlasterPeel / Recalculate* 命令族的业务逻辑全部**内联于消费系统**（渲染域 WP6、set_wall_style 0x1409a2290 的 update_plaster_peel 0x140aaa350 实例化），本文件无独立可恢复业务过程。

### 遗留 U

- register_required_components 0x140990b00 的组件表内容（I）。
- Plaster 组件字段布局（渲染域另行恢复）。
