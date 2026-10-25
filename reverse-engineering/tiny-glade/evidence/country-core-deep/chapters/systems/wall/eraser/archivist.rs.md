# systems/wall/eraser/archivist.rs

> 文件作用：擦除归档器：WallEraserArchivist 累积 original/split/rewire 事件（cache_if_original_wall/shrink_wall/add_split_event/add_fully_erased_event），finish 经 CompoundEditBuilder::build 编译 WallHistoryCompoundEdit；断言 original_wall_state.curve.is_valid()/!before_wall.curve.is_empty()；ChildWalls/WallHistoryEdits。

函数总计 22：业务 7 / 内联无独立符号 9 / 派生胶水 6

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::new` | 0x14092e6e0 | 110 | WallEraserArchivist::new | O | wall\decomp\decomp-0x14092e6e0.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_rewiring_event` | 0x14092e8e0 | 118 | WallEraserArchivist::record_rewiring_event | O | wall\decomp\decomp-0x14092e8e0.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::cache_if_original_wall` | 0x14092ea50 | 133 | WallEraserArchivist::cache_if_original_wall | O | wall\decomp\decomp-0x14092ea50.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::shrink_wall` | 0x14092eff0 | 167 | WallEraserArchivist::shrink_wall | O | wall\decomp\decomp-0x14092eff0.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_split_event` | 0x14092f280 | 187 | 累加/追加 | I | wall\decomp\decomp-0x14092f280.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::add_fully_erased_event` | 0x14092f630 | 224 | 累加/追加 | I | wall\decomp\decomp-0x14092f630.txt |
| `country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish` | 0x14092f990 | 270 | WallEraserArchivist::finish（CompoundEditBuilder::build；!before_wall.curve.is_empty() 断言） | O | wall\decomp\decomp-0x14092f990.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::affected_walls`（行 39）— 待推断
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::has_anything_been_affected`（行 43）— 谓词/校验
- `country_core::systems::wall::eraser::archivist::ChildWalls::remove`（行 73）— 删除/清理
- `country_core::systems::wall::eraser::archivist::ChildWalls::extend`（行 82）— 累加/追加
- `country_core::systems::wall::eraser::archivist::ChildWalls::is_empty`（行 98）— 谓词/校验
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::record_erase_original_wall`（行 259）— 渲染/GPU
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish_shrink_or_split_event`（行 353）— 生命周期控制
- `country_core::systems::wall::eraser::archivist::WallEraserArchivist::finish_erase_event`（行 540）— 生命周期控制
- `country_core::systems::wall::eraser::archivist::WallHistoryEdits::compile_history_edit`（行 633）— 待推断

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`eq` 等 6 项

## 实现方式（算法级）

### WallEraserArchivist 结构（O）
- **new 0x14092e6e0**：0x90B 结构 = 3 个子块 @+0/+0x30/+0x60，每块 `{4×u64 常量(UNK_142aafa60..78), id, value}`；id 来自全局静态计数器（FUN_140d4d740 取静态 + FUN_140d64e00 生成新 id，once-init+自增）——即 3 组 ChildWalls/记录。第 3 块 @+0x60 为第二 hashbrown map（{ptr@+0x60, mask@+0x68}，0x90 步长条目）；+0xc0 区另有一 0x10 步长 map（shrink_wall 首查）。

### record_rewiring_event 0x14092e8e0（O）
- 拷贝 before/after 快照（param_3/param_4 各 0x70B：曲线 Vec3@0、u 数组@0x18 等）→ `FUN_141526910(&out, self+0x30, wall_id, &快照)` 推入 **+0x30 子块**（rewiring 事件列表）。drop 时释放两条曲线数组。

### cache_if_original_wall 0x14092ea50（O 结构 / U 细节）
- 反编译失败（"Unable to find unique hash"，0x168 帧真实函数）。机器码：`+0x18==0 → 早退`；hashbrown 查 self+0x20（0x10 步长，键在 -0x10）；再查 self+0x60（0x90 步长，键在 -0x90）。→ 把墙的原始状态缓存进 archivist。细节字段捕获 U。

### shrink_wall 0x14092eff0（O）
- 首查 0x10 步长 map（affected）→ 命中再查 0x90 步长 map（param_1[0xc]=ptr, [0xd]=mask，键在 -0x90）→ 在条目内数组（0x10 步长，匹配 `elem[2]==wall_id`）中调 `FUN_140a24000`（记录收缩快照）并回写 `{result, 8}`。
- 未命中 → `panic!("no entry found for key")`（archivist.rs:22，UNK_142aaf7e5/142aaff10）+ fmt panic（UNK_142aaff78/142aaff88）。

### add_split_event 0x14092f280 / add_fully_erased_event 0x14092f630（O 结构，I 语义）
- 同 shrink_wall 双 map 查找模式；`FUN_140a24000(&out, &快照, ...)` + `FUN_140fb2220(archivist, wall_id, ...)`（追加 split 段）；未命中同样 `no entry found for key`（UNK_142aafea8 等）。fully_erased 走 `func_0x000140613c20(.., lVar9<<4)` 释放段数组。

### finish 0x14092f990（O，88KB）
- 组装 WallHistoryCompoundEdit：遍历 archivist 记录（**0xf8B/条**，func_0x142923c10 拷贝）与 0x90B map 条目 → `FUN_141466bb0`（曲线切片，`u=(0.2/elapsed)×0.5` 区间）+ CurveU 校验（越界 `Tried to init CurveU...`，NaN `!u.is_nan()`，inf `!u.is_infinite()`）。
- **断言族**：`!before_state.curve.is_empty()`（line 48）、`min_wall_length_u > 0.0`（41）、`before_start_u.u() < before_end_u.u()`（55）、`start_u.u() <= end_u.u()`（42，finish_shrink_or_split_event）；`!before_wall.curve.is_empty()`（47）、`!before_chunk.curve.is_empty()`（48）、`original_wall_state.curve.is_valid()`（54，finish_erase_event）。
- 每事件构造 0x108B 编辑项（func_0x000140613c10(0x108,8)）→ Vec push（FUN_140562870/140ad7990）→ 尾部：`FUN_141696f50`（**CompoundEditBuilder::build**）→ `FUN_140a84ec0(param_3, built, &UNK_142ab00c8=wall_history.rs)`（提交）→ 释放 0xf8B 记录与临时数组（FUN_140b79ff0/140b79810）。
- **纠错**：`!before_wall.curve.is_empty()` 实为两条文案（`before_state` line 48 / `before_wall` line 47），DIGEST §3 合并为一条。
