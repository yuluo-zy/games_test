# 五模块业务逻辑逆向分析总览（tree-tool / water / critters / clutter / audio）

2026-10-13。范围：Tiny Glade 原作 B 主基线（patched EXE，cf048e27…）五个游戏自有模块的反汇编级业务逻辑分析、Rust 恢复草稿与证据。方法论：**先分析每个函数的业务逻辑（调用者泛型签名 / reva 定点反编译 / radare2 RNE oracle 原指令逐条一致），整理成证据文档，再按 PDB 源码骨架实现 Rust；机器码差分仅作校验，不追求逐字节对拍。**

## 交付物索引

### 清单（源文件 / 模块 / 函数边界，A/B 对照）

| 文件 | 内容 |
|---|---|
| `system-water-inventory.json` | water 13 源文件、A/B 模块、函数分组（CPU 状态/历史/GPU/音频） |
| `tree-tool-inventory.json` | tree-tool/forest 源文件与函数分组 |
| `critters-inventory.json` | critters 33 源文件、73 个 B 函数分组 |
| `clutter-inventory.json` | clutter 源文件、26 个 B 业务函数 |
| `audio-inventory.json` | audio 46 源文件、78 个 B 函数分组 |

### 反汇编级分析文档（`-analysis.md` / `-extra-analysis.md`）

| 文档 | 覆盖函数与关键结论 |
|---|---|
| `tree-process-cmds-analysis.md` | process_tree_cmds 三段命令（Move→Insert→Delete）、ECS 签名、命令槽布局、TreeStorage/WaterRaster/ActiveSession 布局 |
| `tree-tool-rest-analysis.md` | despawn_trees（Delete 闭环）、replay_tree、recalc_tree_position、tree_move_bend |
| `tree-tool-extra-analysis.md` | convert_preplaced_trees、reset_tree_tool、ui_place_tree→place_tree 中转链 |
| `water-cpu-analysis.md` | startup_water（角色修正：水位在 WorldRaster<512>）、water_rasterize_from_cpu、world_raster<Water> 链、replay_raster；笔画→栅格公式 `(world/130+0.5)*512` |
| `water-extra-analysis.md` | draw_water 工具分支、water_audio_grid、水位 u8=/255、三闭包 env0/1/2 分派表 |
| `critters-behavior-analysis.md` | duck 五段链、shared 移动/避让、sheep_walk_on_terrain；RNG=Perchance（非 fastrand） |
| `critters-extra-analysis.md` | sim_wren_boids boids 规则（半径/权重/速度）、fish/butterfly、DuckState 1/2/3、TrajectoryPlan 布局 |
| `clutter-placement-analysis.md` | populate 44KB、process 筛选/重叠剔除、add_* 放置规则、AutoClutterState 布局 |
| `clutter-extra-analysis.md` | host_geometry merlon/stitch、auto_clutter_edit 历史、on_windows 跳表、0x68 记录字段 |
| `audio-events-analysis.md` | add_audio_events（16 事件顺序）、wall/building/stairs/music 事件→FMOD 参数映射 |
| `audio-extra-analysis.md` | wall kind 0–23 逐条、building kind 配对、menu/sheep/clutter kind 表、music 公式 |

### 编译验证

- `compile-clutter-audio.json`：`reconstruction/clutter-audio/`（workspace，crate `tg-clutter-audio`）offline cargo check/build/test 全过，5/5 集成测试；clutter 22+3、audio 33+5 unknown/TODO 如实标注。
- critters：`reconstruction/critters/` harness，cargo test 22 通过。
- water：`reconstruction/terrain/crates/tg-terrain/`，水模块 cargo test 9 通过。

### Rust 恢复草稿

- `reconstruction/tree-tool/src/`：process_tree_cmds、despawn_trees、replay_tree、recalc_tree_position_on_terrain_height_change、tree_move_bend、convert_preplaced_trees
- `reconstruction/terrain/crates/tg-terrain/src/`：`water.rs`、`water_audio_grid.rs`（已注册 lib.rs）
- `reconstruction/critters/src/`：duck、shared、sheep_terrain、wren_boids、fish、butterfly
- `reconstruction/clutter/src/`：autoclutter/mod、populate、process、add_rules、manage、merlons、stitches、auto_clutter_edit
- `reconstruction/audio/src/`：events、update、wall_kinds、stairs、music
- `reconstruction/clutter-audio/`：clutter+audio 可编译 workspace

### 工具

- `scripts/binary_baseline/dump_pdb_functions.py`：从 PDB 模块提取函数记录；**注意 llvm-pdbutil `addr` 为 .text 段 offset（十进制），VA = off + 0x140001000**（此前按十六进制解析曾致 VA 全部错位，已修复并重算 303 个 VA）。
- `scripts/binary_baseline/pdb_sources_by_module.py`：源文件流→模块定位。
- 原始反汇编/反编译证据：`evidence/toolchain/{tree-cmds,tree-rest,tree-extra,critters,critters-extra,water-cpu,water-extra,clutter,clutter-extra,audio-extra}/`。
- agent job 定义：`tool-usage/jobs/{tree-cmds,tree-rest,tree-extra,critters,critters-extra,water-cpu,clutter,clutter-extra,audio-extra}/`。

## 业务链闭环摘要

- **tree-tool**：UI→PlaceTreeCmd→place_tree→InsertTreeCmd→process_tree_cmds→SpawnPrefab/despawn_trees；ui_move/delete→历史 TreeEdit(variant 1/2)。
- **system-water**：draw_water(工具 id 15)→world_raster<Water> 链→water_rasterize_from_cpu(GPU 上传)；replay 重跑同链恢复水位。
- **critters**：duck 五段链（plan→move→goals→velocities→ripples）+ boids + sheep 状态机 + 避让。
- **clutter**：populate→filter_clutter_object→process→add_*(gates/windows/birdnests/lantern/tree)→manage；merlon/stitch 宿主几何；auto_clutter_edit 历史。
- **audio**：add_audio_events 16 事件注册；各事件系统按 kind 跳表分派到 FMOD 事件名/参数。

## 剩余 unknown（不伪造，逐项在各文档）

- tree-tool：PlaceTreeCmd 载荷、TreeEdit 判别值跨系统对齐。
- water：mask SSBO 字段（PDB 仅名称）、zeno 闭包内部。
- critters：CritterLocomotion 物种映射。
- clutter：BrickTransformSsbo/StairsFloatingBrickParams 组件级字段、写块栈别名疑点。
- audio：各 kind 对应的 FMOD 资源类型（事件名已逐条）。

## 重放与边界

- 三版 A/B/C 地址不通用；本批全部 B 主基线。`pdb-exports/` 原始 files 导出（约 53MB）未提交，可用 `llvm-pdbutil dump -files <pdb>` 重放。
- 全部为分析证据与恢复草稿，非原作者源码；不因编译通过声称原作等价。
