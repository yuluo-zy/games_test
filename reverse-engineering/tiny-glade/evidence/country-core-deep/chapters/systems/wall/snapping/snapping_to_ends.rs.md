# systems/wall/snapping/snapping_to_ends.rs

> 文件作用：端吸附系统：非 enclosure 公共墙；InProgressCurve::first_raw_input_point 投影墙曲线（get_pos_at_coord/get_pos_at_u/start_pos）到 find_closest_end_to_snap_to（u=0/1）到 mode=2。

函数总计 2：业务 1 / 内联无独立符号 1 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::snapping::snapping_to_ends::snapping_to_ends` | 0x140a6ad80 | 10 | snapping_to_ends（首输入点投影墙曲线→最近端→mode=2） | O | wall\decomp\decomp-0x140a6ad80.txt |

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::systems::wall::snapping::snapping_to_ends::find_closest_end_to_snap_to`（行 36）— 查询/计算

## 实现方式（算法级）

### snapping_to_ends 0x140a6ad80（O，全读 + 机器码核验）

签名（4 参）：param_1 = ResMut\<SnappingMode>（tick 回写），param_2 = InProgressCurve 资源（+0x0 = Option\<WallId> i64::MIN 哨兵、+0x8 首点 ptr、+0x10 点数、+0x80 当前墙 WallId、+0x9e 标志字节），param_3 = 光标资源（Option\<Vec2>：bit0 标志 + x@+4 + y@+0xc），param_4 = &PublicWalls（hashbrown 0x2a8 步长）。

处理链：
1. `mode != 1` → return（仅段吸附候选态才探测端）。
2. `in_progress.+0x0 == i64::MIN`（机器码 `xor eax,eax; cmp [r12],rax; jo` = 溢出标志 = 等于 MIN）→ return（无进行中墙）。
3. 读光标 x/y（fVar51/fVar52）；保存 `cVar31 = snapped@1`。
4. **遍历 PublicWalls，前置 `while snapped == 0`**（snapped 为真时不做端吸附）。每墙：
   - `count>=2 && length>0`（@K+0x18/@K+0x38 = value+0x10/+0x30）失败 → panic（public_walls.rs:50 断言，**4 个内联点**）。
   - `shape tag@value+0x258 != 2` → 跳过（仅 Freehand）。
   - `wall_id == in_progress.+0x80` → 跳过自身；此时改用进行中墙首点 `ptr@+0x8` 作端点（`param_2[2]==0` 即点数空 → 跳过），`+0x9e 标志 & 1` → 强制 snapped=1。
5. **最近端判定**：距离光标 = `SQRT(|光标-首点|²)` vs `SQRT(|光标-末点|²)`（末点 = ptr[count*2-2],ptr[count*2-1]）；首点更近 → 用首点并记 `bVar27=0`，否则末点 `bVar27=1`。**最小距离追踪**：初值 = **1.0e8f（0x4cbebc20 @0x142ad4950）**，`dist < min` 才更新候选（wall id + 端点 + 端标志）。
6. 迭代后：`min >= 1.5` 或 `bVar29==2`（从未命中）→ return。
7. **投影**：对最佳候选墙 `Curve::get_coord_at_u<Vec2,2D>`（FUN_141467ce0，入参 curve@value+0x38，返回 {段索引 u, 分数 t}）；越界检查（曲线点数 @value+0x48）失败 → panic_bounds_check；`t∉[0,1]` → panic（curve_u.rs:42）；`!u.is_nan()` → panic（curve_u.rs:29）。
8. **3D 吸附点**：`lerp(point[u], point[u+1], t)`（12B Vec3 元素）→ {x,y}（u64 打包）+ z（f32）。
9. **写 SnappingMode**：`mode=2; snapped=wall+0x29e 字节; pos（movlps [r14+4] 8B + movss [r14+0xc]）; wall_id@+0x10; end@+0x18 = bVar29`；tick 回写。

panic 清单：`simplified_curve_2d.is_valid()` 断言 ×4（UNK_142ad52f8/52c8/5370/52e0 域文件）、`unwrap_failed`（0x142ad53e8/5388 域）、`panic_bounds_check`（curve/mod.rs）。

### find_closest_end_to_snap_to（行 36，O 并入上述第 5 步）
逐墙双端距离比较 + 全局最小跟踪 = 该内联函数本体。

### 常量表
| 常量 | 值 | 含义 |
|---|---|---|
| 0x142ad4950 | 1.0e8f | 最小距离初值（哨兵） |
| 0x1429258d0 | 1.5f | 端吸附距离阈值（min < 1.5 才吸附） |
