# systems/wall/eraser/wall_eraser.rs

> 文件作用：徒手墙擦除系统：拖笔选段到 WallEraserArchivist::affected_walls/has_anything_been_affected 到 InnerWalls::get_unwrapped/update_wall_state(_curve/_enclosure) + new_wall；determine_freehand_height_ty、wall_height_based_on_length、adjust_wall_height_if_erased_flat_roof_w_no_rims（PublicWalls::roof_entity ×4）；ChildWall::new 子墙。

函数总计 10：业务 1 / 内联无独立符号 3 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::wall_eraser::wall_eraser` | 0x140accab0 | 0 | wall_eraser（拖笔选段 + archivist + ChildWall::new + 平屋顶高度补偿） | O | wall\decomp\decomp-0x140accab0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::wall_eraser::adjust_wall_height_if_erased_flat_roof_w_no_rims`（行 549）— 待推断
- `country_core::systems::wall::eraser::wall_eraser::determine_freehand_height_ty`（行 571）— 待推断
- `country_core::systems::wall::eraser::wall_eraser::wall_height_based_on_length`（行 599）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 6 项

## 实现方式（算法级）

### wall_eraser 0x140accab0（O，126KB）
- **初始化/模式**：`FUN_14092df50(资源, &"eraser 参数"?, 6, 6, 3, &stroke)`（笔参数）；模式字节 bVar56 ∈{0,1,2,3} switch（0=返回，3=释放处理）。
- **拖笔选段（核心）**：对鼠标 stroke 点逐点 `func_0x000140c904d0`（曲线投影 u）→ 累积 `Vec<{u, f32}>`（8B 元素，cap-first FUN_140562e70 扩容）；`间距阈值 0.1`（_UNK_142925984）以下合并相邻点（`(u_next-u_prev)*fVar123 < fVar4` → 并入前段，否则压缩）。u 校验：越界 → `assertion failed: (0.0..=1.0).contains(&t)`（curve_u.rs，wall_eraser.rs:42，UNK_142aa53ed/142aa5468）。单点 stroke（`len==1 && u==0`）→ 清空早退（不擦除）。
- **每段处理**（pfStack_120==1 单段路径）：写 **0x20B 段事件** `{u64, WallId, stroke 数据, u32, 0x202 u16}`（FUN_140562c30 扩容）→ `FUN_1413e7230`（写事件）+ `decorator_cull_allow_any(0x140b2bc00)` + `FUN_14092ea50`（cache_if_original_wall）→ `FUN_141466bb0(out, 墙曲线, u_lo, u_hi)`（**曲线切片**）→ 断言 `proposed_curve.is_valid()`（wall_eraser.rs:43，UNK_142ae0b10）→ `FUN_14092eff0`（**shrink_wall**）。
- **ChildWall::new（内联）**：拷贝墙曲线数组（uVar11 点×8B 或 ×4B）→ 按 `shape tag(value+0x38)-2 clamp ≤2` 三路构建子墙参数 → `FUN_140aaa090`（**update_wall_state_curve**）+ `FUN_140aa9610`（**enclosure tag→2 Freehand**）。
- **墙高更新**：`FUN_140aa9bc0(inner, wall_id, &高度状态)`（_try_update_state<update_wall_state>）。高度计算：
  - **wall_height_based_on_length**（行 599，内联）：`t=clamp01(func_0x000140c904a0())`；`height = lerp(0.75, 14.0, t)`（0.75@0x1429edb90、14.0@0x142a7de00）；`new = min(当前高 value+0x54, height)`。
  - **adjust_wall_height_if_erased_flat_roof_w_no_rims**（行 549，内联）：预置墙（tag<2）且有屋顶 → `roof+0x3d!=0 && roof+0x30<=0.1` 时 `height += -0.79`（_UNK_142a7e320）。roof_entity 调用 ×4（两处分支各 2 次，含 Debug 开关 cRam0000000143634d90）。
- **释放处理**：`FUN_14092f990(archivist, ...)`（**finish**）→ 遍历 affected（0x10 步长）写后续列表；`.collect` 走 std iterator（/rustc/05f9846f…iterator.rs，UNK_142ae0658）。
- **纠错**：`determine_freehand_height_ty`（行 571）与高度 clamp 合并于同一高度更新路径，非独立函数；roof_entity 实际 ×4（旧述 ×4 ✓ 但分布不同）。
