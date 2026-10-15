# systems/wall/snapping/mod.rs

> 文件作用：吸附模块根：SnappingMode{mode u8@0, snapped bool@1}（mode 1=段吸附 2=端吸附）+ +0x20 声音状态字节；is_active/position_to_snap_to。

函数总计 4：业务 1 / 内联无独立符号 2 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::SnappingMode::new` | 0x140ade610 | 90 | SnappingMode::new（11B {mode@0,snapped@1}） | O | wall\decomp\decomp-0x140ade610.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::snapping::SnappingMode::is_active`（行 67）— 谓词/校验
- `country_core::systems::wall::snapping::SnappingMode::position_to_snap_to`（行 74）— 吸附/数值

## 派生胶水（derive/宏生成；不计独立业务）

`clone` 等 1 项

## 实现方式（算法级）

### SnappingMode 布局（O，综合 snapping_to_segments/ends/maintain/audio/new 五处写读）

| 偏移 | 字段 | 证据 |
|---|---|---|
| +0x00 | mode u8（0=无 / 1=段吸附候选 / 2=端吸附 / 3=段吸附生效） | 各系统 `mov byte [rX], imm` |
| +0x01 | snapped u8 | snapping_init 末 `puVar6[1]=count*8!=last_idx`；to_segments/ends 从 wall+0x29e 复制 |
| +0x04 | u32/f32 复用区：mode 3 = 地形高度（`TerrainHeightsData::sample_at_world_xz` 0x1409e9640 返回值）；mode 2 = 吸附点 y | to_segments `movss [rbx+4],xmm6`；to_ends `movlps [r14+4]` |
| +0x08 | u64 复用区：mode 3 = 吸附点 Vec2{x@8,y@0xc}；mode 2 = 吸附点 x | to_segments `movsd [rbx+8]`；to_ends `movlps [r14+4]` 跨 +4..+0xb |
| +0x0c | f32：mode 3 = 吸附点 y；mode 2 = 吸附点 z | to_segments `movss [rbx+0xc]`；to_ends `movss [r14+0xc]` |
| +0x10 | WallId u64 | 两系统 `mov qword [..+0x10]` |
| +0x18 | 端 id u8（仅 mode 2：0=首端近、1=尾端近） | to_ends `mov byte [r14+0x18],bl` |
| +0x20 | 上一帧发布 mode u8（声音去重 + 快照区起点） | `SnappingMode::new` 写 0；snapping_maintain 写当前 mode；snapping_audio 读作去重 |
| +0x20..+0x38 | 快照区（snapping_maintain 打包） | {mode@0x20, snapped@0x21, u64@0x24, f32@0x2c, u64@0x30, byte@0x38} |

**mode 2 vs 3 的 +4..+0xc 复用冲突**：段吸附写 {height@4, x@8, y@0xc}，端吸附写 {x@4..+0xb(u64), z@0xc}。两处 store 机器码均确认。若为同一 struct，则该区为按 mode 复用的联合语义（I，源类型名 U）；或 Ghidra 对 mode 2 的 store 基址偏移误判（U）。

- `SnappingMode::new` 0x140ade610（O，11B）：`mode=0; +0x20=0; ret`。**不初始化 snapped@1**（重建代码 `snapped:false` 与此不符，为未初始化）。
- `is_active`（行 67，I）：mode != 0（见 check_for_disable 的 `cVar1==2/3` 判别路径）。
- `position_to_snap_to`（行 74，I）：读 +8 区吸附点；重建 `Vec3::ZERO` 占位需按 mode 读 {x,y(+z)}。

### 模式状态机（O，跨系统）

```
snapping_init (mode=1, snapped=首段投影判定)
  ├─ snapped==0 → snapping_to_ends 遍历 PublicWalls → 命中 → mode=2（端吸附）
  └─ mode==1 → snapping_to_segments 查 WallSpatialHash → 命中 → mode=3（段吸附）
check_for_disable_snapping：mode∈{2,3} → 写 DisableSnapping 事件 +（多数路径）mode=0
snapping_maintain：把当前 mode/位置打包到 +0x20 快照（供 snapping_audio 去重）
snapping_audio：mode 2/3 且 +0x20 未重复 → 写 PlayWallUiSound
```
