# groupA RE-ANALYSIS-2 — forest 6 栅格采样身份（task ②）

dbid: tg_xa1 · 2026-10-16 · 补充证据: `forest/decomp/decomp-0x141bf16c0.txt` + 机器码（rne-disasm）+ `grid-decomp-140ae4c20.txt`

## 结论（O）

manage_tree_prefabs / convert_preplaced_trees 的 **6 路栅格采样**：

| # | 采样输入槽 | 身份 | 证据 |
|---|---|---|---|
| 1 | auStack_290（= [rbp-0x48]） | **PathRastersResources 槽 1**（u8 栅格视图 {ptr,end}） | 0x141bf19d5 `call 0x140ae4c20`（as_float_grid）输出；0x141bf1c47 采样 |
| 2 | auStack_280（= [rbp-0x38]） | **PathRastersResources 槽 2** | as_float_grid 输出 [1..2]；0x141bf1c67 采样 |
| 3 | auStack_270（= [rbp-0x28]） | **PathRastersResources 槽 3** | 同上；0x141bf1c87 采样 |
| 4 | auStack_260（= [rbp-0x18]） | **PathRastersResources 槽 4** | 同上；0x141bf1ca7 采样 |
| 5 | uStack_208（= [rbp+0x40]） | **param_12 资源**（WorldRaster 形容器，经 0x14067dd80 视图）—— **WaterRaster 候选（I）** | 0x141bf1995/19c9 访问器；0x141bf1cc7 采样 |
| 6 | uStack_218（= [rbp+0x30]） | **param_13 资源**（同形）—— **GardenRaster 候选（I）** | 0x141bf19e4/1a11 访问器；0x141bf1ce7 采样 |

## 关键函数身份（O，机器码级）

- **0x140ae4c20 = PathRastersResources::as_float_grid**（draw_path 章节同证）：`(out: &mut [u64;8], srcs: &[u64;4])`—— 从 **4 个源指针**（param_2[0]/[4]/[8]/[0xc]）各取 {ptr,end} 栅格切片写入 out[0..7]；每个源为同一容器形 {Vec{ptr@+8 或 +0x20, end@+0x10 或 +0x28}, 选择字节@+0x298}。**并非"转 float"**——旧名误导；实为 4 路切片收集。
- **0x140a14df0 = WorldRaster<512>::bilinear_sample_at_world_xz**（O）：`(grid_ptr_slot, x, y, u32, 130.0f32(0x43020000))`；坐标/130 → clamp[0,1] → ×512（`(w/130+0.5)*512` 即章节 ws_to_px 行 39）→ 双线性（权重表 0x1429258e0..8ec）→ 返回 `grid[+0x201]` 处字节；0x14362c308 全局分硬件/软件采样路径。512 宽栅格 = WorldRaster<512> composite/scratch（章节布局：各 0x40000B，脏位图+0x98，selector+0x298，脏标志+0x299）。
- **0x14067dd80 = WorldRaster 视图访问器（as_float_grid 内联实例化）**：`(res) → {ptr,end}`，按 +0x298 selector 选 composite 视图；PDB 名 FinalizedHoleStorage::holes 系共享单态错位，登记。
- **0x140ad51a0 = PlacementDataSsbo::position**（<40B NotFound，手解候选）：取候选 0x50B 条目位置 {x,y}（所有树系统共用）。

## 语义契约（O）

- 排除面（采样 5/6）：任一 `> 0` → 候选 flag=0（不处理）—— 树不落在水面/花园（I：WaterRaster=槽4、GardenRaster=槽6，按 world_raster_defs 六栅格清单 {PathRaster, Cobblestone1-3, WaterRaster, GardenRaster} 推断）。
- 密度面（采样 1-4）：`density = clamp(Σ4, 0, 1)`；`density > 0 || is_too_close_to_shape → 跳过`—— 树只种在 4 路地面栅格全 0 且离形状 ≥ 7.0 处。
- 采样尺度常量 130.0 与 WorldRaster::ws_to_px `(w/130+0.5)*512` 一致（章节 O）—— 全链路闭环。

## 交叉登记

- PathRastersResources（systems/paths/draw_path，章节 O）：as_float_grid 4 路；draw_path 域含 DrawState（0x140ae4bd0）—— tree_canvas/water 画笔共用。
- convert_preplaced_trees（0x14226be70）同 6 路结构（auStack_2b8/2a8/298/288 + uStack_258/268），槽位同义。

## U 项

- param_12/param_13 精确资源名（WaterRaster/GardenRaster 为 I；确认途径：bevy 注册参数序或 AllWorldRasterRead 槽序）。
- 0x140ad51a0 函数体（未建函数）。
