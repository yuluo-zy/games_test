# systems/wall/snapping/snapping_to_segments.rs

> 文件作用：段吸附系统：raw_input_curve_length_ws；closest_segment_to_snap_to（get_curve_segment_from_u/get_tangent_at_u + distance_segment_point）最近段到 mode=1。

函数总计 4：业务 1 / 内联无独立符号 1 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_to_segments::snapping_to_segments` | 0x140a6b5e0 | 21 | snapping_to_segments（raw 长度 + closest_segment_to_snap_to → mode=1） | O | wall\decomp\decomp-0x140a6b5e0.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::snapping::snapping_to_segments::closest_segment_to_snap_to`（行 53）— 查询/计算

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

### snapping_to_segments 0x140a6b5e0（O，全读 + 机器码核验）

签名（6 参）：param_1 = ResMut\<SnappingMode>（tick 回写），param_2 = 资源（+0x0 = i64::MIN 哨兵，非 MIN 才继续），param_3 = 光标资源（Option\<Vec2>：bit0 + x@+4 + y@+0xc），param_4 = &PublicWalls，param_5 = &WallSpatialHash，param_6 = &TerrainHeightsData。

处理链：
1. `mode != 1` → return（仅段吸附候选态）。
2. `param_2.+0x0 == i64::MIN` → return。
3. 保存 `snapped`（acStack_61[0]）；读光标。
4. **AABB 查询**：以光标为中心 ±**8.0f** 构造包围盒（常量 0x142ab6630/34/38/3c = {-8,-8,+8,+8}：0xc0100000/0x40100000）→ `WallSpatialHash::query_with_aabb_clamped`（0x140aa5a10）。
5. 对每个查询结果（候选点 + 墙 id@+0x20）：
   - `SQRT(|候选-光标|²) >= 1.5`（0x1429258d0）→ 跳过（继续下个候选）。
   - 以墙 id 在 PublicWalls hashbrown 探针（步长 0x2a8）；`count>=2 && length>0`（value+0x10/+0x30）失败 → panic（public_walls.rs:50，UNK_142ad5418 域）。
   - **命中即写**：`mode=3; snapped=wall+0x29e 字节; height@+4 = TerrainHeightsData::sample_at_world_xz(光标)（0x1409e9640）; pos@+8 = 候选点（movsd）; wall_id@+0x10`；return。
6. 无命中 → 清理临时迭代器（func_0x000140613c20 ×2）后 return。

**纠错**：章节"closest_segment_to_snap_to（get_curve_segment_from_u/get_tangent_at_u + distance_segment_point）"过细——实际是"空间哈希 AABB 查询 + 1.5 距离筛选"，无 get_tangent_at_u 调用；候选墙即为距离最近者（查询按 AABB 返回）。

常量：
- AABB 半宽 **8.0f**（0x142ab6630 区），距离阈值 **1.5f**（0x1429258d0）。

panic：`simplified_curve_2d.is_valid()`（public_walls.rs:50，行 0x32）、`unwrap_failed`（0x142ad53e8/142ad5418 域）。
