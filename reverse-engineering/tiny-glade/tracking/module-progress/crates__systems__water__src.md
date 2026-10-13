# crates/systems/water/src

状态：部分候选实现。完整模块验收：未通过。

B原接口已观察/已分析/未触及：未盘点。[逐接口进度](../../INTERFACE_PROGRESS.md)。

定义归属：private_game_source。

WaterRaster CPU状态、world_raster/replay/上传调用、water_audio_grid。

## 接线与验证

接线：not_established。

五模块README记录候选9测试；没有完整水域原作运行等价。

## 已恢复的函数和类型范围

- draw_water工具分派分析
- startup_water、water_rasterize_from_cpu、replay_raster/replay_water
- world→512网格映射、水位u8/255、audio-grid更新

## 明确缺口

- zeno闭包内部、mask SSBO字段和完整draw/history producer
- 真实GPU上传、water-render consumer与系统生命周期

下一闭包：zeno闭包内部、mask SSBO字段和完整draw/history producer

## 证据

- [evidence/multi-baseline/README.md](../../evidence/multi-baseline/README.md)
- [evidence/multi-baseline/water-cpu-analysis.md](../../evidence/multi-baseline/water-cpu-analysis.md)
- [evidence/multi-baseline/water-extra-analysis.md](../../evidence/multi-baseline/water-extra-analysis.md)

## 原作文件逐项覆盖

| 原作文件 | 版本 | 对应恢复文件及映射依据 | 原函数证据 | 文件级状态 |
|---|---|---|---|---|
| `crates/systems/water/src/draw_water.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/water.rs](../../reconstruction/terrain/crates/tg-terrain/src/water.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/water/src/lib.rs` | A,B,C | 未建立精确映射；模块级恢复文件另列 | 全集/逐函数映射待核对 | not_assessed；not_file_level_verified |
| `crates/systems/water/src/record_water_history.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/water.rs](../../reconstruction/terrain/crates/tg-terrain/src/water.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |
| `crates/systems/water/src/water_audio_grid.rs` | A,B,C | [reconstruction/terrain/crates/tg-terrain/src/water_audio_grid.rs](../../reconstruction/terrain/crates/tg-terrain/src/water_audio_grid.rs)：reviewed_partial_module_mapping | 全集/逐函数映射待核对 | candidate_present；not_file_level_verified |

## 本机恢复文件与Rust定义

下面是声明扫描，不代表这些函数全部对应原作、全部完成或全部已通过验证。测试模块的常规尾部不计入。

### reconstruction/terrain/crates/tg-terrain/src/water.rs

[reconstruction/terrain/crates/tg-terrain/src/water.rs](../../reconstruction/terrain/crates/tg-terrain/src/water.rs)

SHA256：`e4bcdf13f176d061d64a7644aa6f595f0d0e43f512fd8d4fa802a8ca39fc43d9`；缺口注释标记数：10。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `point_to_grid_xz` | 44 | 未由扫描证明；以证据限定范围为准 |
| `build_stroke_commands` | 63 | 未由扫描证明；以证据限定范围为准 |
| `new` | 104 | 未由扫描证明；以证据限定范围为准 |
| `copy_composite_to_scratch` | 119 | 未由扫描证明；以证据限定范围为准 |
| `commit_scratch_to_composite` | 126 | 未由扫描证明；以证据限定范围为准 |
| `copy_to_staging` | 133 | 未由扫描证明；以证据限定范围为准 |
| `bilinear_sample_at_world_xz` | 148 | 未由扫描证明；以证据限定范围为准 |
| `replay_restore_water` | 179 | 未由扫描证明；以证据限定范围为准 |
| `select_write_mode` | 222 | 未由扫描证明；以证据限定范围为准 |
| `coverage_to_alpha` | 232 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L6：//! 只恢复已证实的水域 CPU 状态链业务规则；所有未知项以 `// [unknown]` 标注，不伪造。
- L31：/// `WorldRaster::new` 填一块 0x4000B 初始块（初始水位），解码语义 [unknown]。
- L54：/// [unknown] 后 16B 字段语义（zeno 依赖，未还原）。
- L91：/// [unknown] 栅格行数/总尺寸；composite/scratch 以 Vec<Cell> 建模仅为近似（布局非 ABI 复刻）。
- L99：/// 512B staging（与脏块共用区，见证据）；[unknown] 精确语义。
- L132：/// [unknown] 原实现用 DirtyTiles::retain 增量复制 + parking_lot Mutex；这里只表达语义。
- L178：/// [unknown]：权重 fVar5 与栅格 u8 值的具体换算；Curve/zeno 依赖实现不在此恢复。
- L186：// [unknown] 实际以闭包经 zeno::mask::Mask::blend_into 写入栅格。
- L189：// [unknown] brush 来自 ActiveRasterStroke/BrushSizes。
- L210：///   `a = 覆盖度>>9` clamp 0xff（zeno 覆盖度定点刻度 [unknown]）。
### reconstruction/terrain/crates/tg-terrain/src/water_audio_grid.rs

[reconstruction/terrain/crates/tg-terrain/src/water_audio_grid.rs](../../reconstruction/terrain/crates/tg-terrain/src/water_audio_grid.rs)

SHA256：`25726ff5416297e1c4a60be915fd537ec290084d3c2fcb81bc72e0f762f5ce2b`；缺口注释标记数：3。

| Rust定义 | 定义行 | 原名/等价状态 |
|---|---|---|
| `frame_bit` | 37 | 未由扫描证明；以证据限定范围为准 |
| `update_cell` | 46 | 未由扫描证明；以证据限定范围为准 |

缺口注释摘录（最多30条）：

- L5：//! 本模块不是 ECS/ABI 复刻，只恢复已证实的水域音频网格业务规则；所有未知项以 `// [unknown]`
- L21：/// +0x10：2×u16 入水连续计数（与 mask 同增减）。[unknown] 双计数的精确语义。
- L31：/// intersects_any 参数（XMM2=1.0 @0x142925904；R9D=0x32a3 标志位掩码）。[unknown] 语义。
