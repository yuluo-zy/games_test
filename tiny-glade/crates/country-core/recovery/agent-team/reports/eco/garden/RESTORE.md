# restorer-eco — garden 模块 RESTORE 报告

日期：2026-10-15
状态：review_passed（限定，中央已裁决）
涉及文件：`systems/garden/*`、`startup/startup_garden_brush.rs`

## 一、原函数 → Rust 位置 → 证据

| 原函数 | 地址 | Rust 位置 | 证据 |
|---|---|---|---|
| systems/garden.rs impl$0::build | 0x140b6d290 | garden/mod.rs:111（`build`） | decomp-0x140b6d290 |
| draw_garden | 0x140ae4620 | garden/draw_garden.rs:39 | decomp-0x140ae4620 |
| record_garden_history | 0x141695e10 | garden/record_garden_history.rs:14 | decomp-0x141695e10 |
| startup_garden_brush | 0x1415f87c0 | startup_garden_brush.rs:29 | decomp-0x1415f87c0 + read_memory 字符串 |

## 二、实现说明

- **draw_garden（0x140ae4620）**：模式分发已核对——工具 id-6<0xf 且 id-5∈{4,0xc}（9=橡皮 / 17=花丛）；`AppMode::Garden` → name "garden"、arg 5；`AppMode::Eraser(All|Garden)` → name "eraser"、arg 9、半径 = inverse_lerp(0.3, 2.0, eraser_multiplier)→lerp(1.0, 4.0, t)。**章节"半径=笔刷+1.5"实为引脚高度**（decomp：`fStack_a0 = cursor_y + 1.5`，常量 0x1429258d0 是高度加量，非半径加量）；当前实现以 `position.y + 1.5` 作引脚高度、`radius * 0.5` 作圆圈半径，与机器码一致。reviewer 已确认。
- **record_garden_history（0x141695e10）**：消费 RecordGardenEditToHistoryCmd，走 `record_raster_history(&mut strokes, 5, &mut history, |stroke| EditType::Garden(stroke))`，失败写 HistoryRecordingFailed。与 0x141695e10 对上。
- **plugin build**：注册事件（register_garden_events）+ Draw→Record 链（add_garden_cpu_systems）+ 回放反馈路由（route_garden_replay_feedback）。原 decomp 是 Bevy 框架类型注册环，本 crate 以 Schedule/World 等价注册。GardenRaster 资源由 world-raster owner 注册（需 DirtyTilePublisher），build 不重复插入。
- **startup_garden_brush**：网格字符串 read_memory 核实——`assets/meshes/garden_stone.json`（0x142c7d468）→ 名 `garden_stone`（0x142c7d487）；`assets/meshes/lowpoly_flower.json`（0x142c7d4de）→ 名 `garden_flower_03`（0x142c7d4ff，**纠偏章节"garden_lowpoly_flower"**）。shader 两个（garden_stone/...raster.hlsl、garden_flower_03/garden_lowpoly_flower.raster.hlsl）+ 2 个 spawn 记录（FUN_1412dd460 / FUN_1412d9de0）依赖渲染管线，登记待接线。

## 三、仍缺失/待取证
1. garden 插件的原 Bevy App 注册（系统集/调度器身份）未逐条复刻（框架耦合，等价注册）。
2. 回放链 `systems/history/replay_garden.rs`（薄包装）未在本 crate 建文件——由 `route_garden_replay_feedback` + `replay_raster` 泛型承担；原 5 函数映射登记于此。
3. startup_garden_brush 的 shader/spawn 记录待渲染管线接线。

## 四、inventory 建议
- `systems/garden.rs`、`systems/garden/draw_garden.rs`、`systems/garden/record_garden_history.rs`、`startup/startup_garden_brush.rs` 提升为 `present_requires_semantic_review`。
- 建议合同登记：draw_garden 工具 id 分发 + 半径/引脚高度公式；record_garden_history slot 5。
- 建议 central 在 contracts.json 登记"章节半径+1.5 纠偏为引脚高度"的裁决。
