# utils/float_grid.rs

> 文件作用：**世界 XZ 双线性采样浮点网格（游戏自有 utils crate；共享 utils 已有同算法恢复
> 版，差异见下）**。机器码锚点：`bilinear_sample_at_world_xz` 实例 `RasterBufferRef<512>`
> @0x140a14df0、`TerrainHeightFloatGrid` @0x140257ba0；调用约定
> `(grid, x, z, extent_x, extent_z)`，调用点恒 `130.0`（0x43020000 同时出现在 xmm3 与栈上）。
> 算法：`p = (z/ez, x/ex) + 0.5` → 512×512 栅格（`u8/255.0` 格点）双线性插值，
> `lerp(lerp(A,C,fz), lerp(B,D,fz), fx)`（`utils::lerp` @0x140c904d0 = `a+(b-a)*t`）。
> 入口全局字节开关 `*0x14362c308`（CENTERED_PATH，**写入者 = load_game 0x14146dc92**（round3 O），
> 目标字节 0x143638fac 初值 0）选两条路径。

## 核心算法（双路径）

```
世界范围：±65 → p = 世界/130 + 0.5 ∈ [0,1]；栅格 512；格点 u8/255（或地形高度读取）

开关 == 0（"向量"路径，默认）：
  t = clamp(p,0,1)*512（NaN→0，maxss/minss 序列）
  i = min(sat_u32(t), 510)；i1 = i+1；frac = t - i（t==512 时 frac=2，外推原样保留）
开关 != 0（"居中/texel-center fix"路径）：
  t = min(p,1)*512 - 0.5（NaN→1.0 → maxss(0)  NaN→0）
  i = sat_u32(t)；i0 = min(i,511)；i1 = min(i+1,511)；frac = t - i
两路径格点顺序与 lerp 顺序相同：A=(row0,col0) B=(row0,col1) C=(row1,col0) D=(row1,col1)，
row = z 轴、col = x 轴（idx = row<<9 | col）
```

## 与共享 `utils`（shared-foundation/crates/utils/src/float_grid.rs）对照

共享版 = 同一 B 机器码的**等价恢复**，差异只在 API 形状：

| 项 | country-core（游戏机器码） | shared-foundation utils | 裁决 |
|---|---|---|---|
| 入口 | 自由函数 `bilinear_sample_at_world_xz(grid,x,z,extent_x,extent_z)` + 全局开关 `*0x14362c308` | trait `FloatGridSampler::bilinear_sample_at_world_xz(&self, xz:Vec2, world_extent:Vec2, sampling_fix:bool)` | 语义等价；`sampling_fix` = CENTERED_PATH |
| 路径选择 | 运行时读全局字节（初值 0 → 向量路径；**load_game 0x14146dc92 写入 = 配置字段 @+0xc0 >= 0x1a**，round3 O） | 编译期 bool 参数 | 行为一致（默认向量路径；load_game 可按存档配置切居中路径） |
| 向量路径 | `maxss(0)→cmpord 屏蔽 NaN→minss(1)→cmpunord`；NaN 整体 → 0 | `v.max(0.0).min(1.0)*n`，**Rust max/min 保 NaN**（frac=NaN，仅 floor=0） | ⚠️ NaN 边界有差异：B 输出 0，共享版输出 NaN（共享注释已自认 "clamp preserves NaN"）——重建应按 B 显式 NaN→0 |
| 居中路径 | `minss(p,1)`（NaN→1）+ `*512-0.5` + `maxss(0)`（NaN→0） | `if v<1.0 {v} else {1.0}` + `.max(0.0)` | min 语义一致；`.max` 的 NaN 处理同左注 |
| 栅格面 | `FloatGridCells::cell(idx)`，u8/255 | `FloatGrid::load_index_unchecked` + `ByteFloatGrid`（u8/255） | 等价 |
| 世界范围 | 常量 130.0（调用点内联） | 调用方传参 | 等价 |
| lerp | `utils::lerp` @0x140c904d0 | 内联 `a+(b-a)*t` | 一致 |

结论：共享 utils 的 float_grid 可作为 country-core 版替换（§4.1 去重方向成立），但需
补 NaN→0 语义与 `CENTERED_PATH` 开关（**写入者已找到：load_game 0x14146dc92 `setge [rax]`，
值 = 配置字段 +0xc0 >= 0x1a**（round3 O）；`sampling_fix` 由 load_game 调用点接线）。
