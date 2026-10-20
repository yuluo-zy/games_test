# 原版 Plugin 清单与宿主接线核对（2026-10-20）

方法：Ghidra 库（`reverse-engineering/tiny-glade/scripts/reverse_tools/linux/`，PE `.pdata` 全量建函数 + PDB 命名）里枚举 `<X as Plugin>::build` 共 46 个，用 PyGhidra 脚本按地址顺序读出每个 `build` 的 `add_systems / in_set / run_if / register_event` 调用，避免反编译对移动值的顺序歧义。

## 本次接入（宿主 `tiny-glade/src`）

| 原插件 | build 地址 | 宿主文件 | 原版调度（机器码顺序） |
|---|---|---|---|
| `system_water::WaterPlugin` | 0x1406501d0 | `water_plugin.rs` | `record_water_history` → set 0x10 单独 add_systems；`update_water_audio_grid` → set 0x22 + `run_if(Option<Res<WaterAudioGrid>>)` + set 0x24；`WaterRaster`（与 history_runtime 共用保护式构造）；3 个事件 |
| `system_glade_border::GladeBorderPlugin` | 0x140650ab0 | `glade_border_plugin.rs` | `(display_glade_border, feedback_input_out_of_border).chain_ignore_deferred().in_set(0x24)`；2 个事件 |

`draw_water` 不属于 WaterPlugin，原版在别处注册（工具/模式层），本次未挂。重建 crate 自带的 `add_*_cpu_systems` 与原版不同（用自定义 stage、缺 set 号），宿主不使用它们。

## 已确认但未解决的断链

- **`GladeBorder` 资源在生产代码中没有任何创建者**，只在测试里手工 `insert_resource`。原版的消费者（`ui_move_shapes`、`ui_draw_terrain`、`draw_garden`、`draw_path`、`ui_create_rectangle/circle`、`draw_water`、`display_glade_border`）都要读它；`display_glade_border` 现已挂入主循环，第一帧会因缺资源而参数校验失败。原版创建者尚未定位（符号里没有构造函数，是泛型 `insert_resource<GladeBorder>`；`glade_startup_walls` 0x140a92670 只建墙网格，不是它）。
- `update_water_audio_grid` 需要 `Res<Raycasting>`，同样没有生产创建者；该系统受 `WaterAudioGrid` 存在性门控，`WaterAudioGrid` 也无生产创建者，所以目前恒不运行。
- 原版没有 Timberframe/Onboarding 的 Plugin：`timberframe` 三个系统和 `system_onboarding` 的 49 个函数由别处注册，未在 46 个 Plugin 里出现。

## 验证边界

只做了 `cargo check -p tiny-glade --lib --features country-core/native-audio`（通过，Cargo.lock 仅增加 tiny-glade→system-water/system-glade-border 两条依赖边）。Linux 上原版 `fmod.dll` 无对应 `libfmod.so`，宿主 bin/test 无法链接，端到端（`original_gameplay_chain`）未运行。
