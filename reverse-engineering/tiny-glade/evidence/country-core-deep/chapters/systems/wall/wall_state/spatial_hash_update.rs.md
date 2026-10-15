# systems/wall/wall_state/spatial_hash_update.rs

> 文件作用：空间哈希同步系统：OnWallChanged 到按 simplified_curve_2d 逐点 get_pos_at_coord/sample_at_coord 建段到 add_segment（缺墙 panic 'SpatialQuery received OnWallChanged with wall index'）；OnWallDeleted 到 remove_wall_id。断言 (0.0..=1.0).contains(&t)。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_state::spatial_hash_update::spatial_hash_update` | 0x140a8fd90 | 0 | spatial_hash_update（EventReader×2+Res<PublicWalls>+ResMut<WallSpatialHash>；simplified_curve_2d→add_segment/remove_wall_id） | O | wall\decomp\decomp-0x140a8fd90.txt |

## 实现方式（算法级）

### spatial_hash_update 0x140a8fd90（4 系统参数）

**参数面**（O）：param_1=EventReader<OnWallChanged>（行 209–233 事件存储遍历），param_2=EventReader<OnWallDeleted>（行 247–274 第二次事件存储遍历），param_3=Res<PublicWalls>（`plStack_158=*param_3`，0x2a8 map，`plStack_158[3]`=mask 判空），param_4=**ResMut<WallSpatialHash>**（行 239 `*(u32*)(param_4+0x1c)` tick 回写 ✓，`uStack_140=*param_4`）。

**OnWallChanged 路径**：
1. 逐 16B 事件行（`lVar29+0x10` 步进），取 `wall_id = *(u64*)(row+8)`（EventInstance {id@0, payload@8}）。
2. PublicWalls 0x2a8 hashbrown 探针查表（行 288–386，`FUN_1405b0d70` hash + SIMD ctrl 组比对）：
   - 未命中（probe 空槽耗尽）→ 若全局 `*puRam000000014362c3a8 > 1` 则 fmt panic **`SpatialQuery received OnWallChanged with wall index {}`**（`func_0x00014067dd80(&UNK_142ad94c8)` 前缀串，`func_0x000140d4a330` 格式化，行 389–411）。
   - 命中但无效（`value+0x10 < 2 || value+0x30 <= 0.0`）→ **panic `assertion failed: w.simplified_curve_2d.is_valid()`**（`FUN_1428d9430(&UNK_142ada193,0x32,&UNK_142ad93b0)`，行 630）。
   - 有效 → **`FUN_140aa5360()` = remove_wall_id**（行 345，先清旧段）；点计数 `uStack_190 = count-1`，为 0 则跳过该墙。
3. **逐段重采样**（点计数>1 时，行 445–607）：
   - 源数据：raw 曲线点 Vec<2×f32, 8B>（`lVar24-0x298`）、高度 Vec<4B>（`-0x280`）、3D 曲线 Vec<12B>（`-0x260`，读 `+4` 偏移）、simplified_curve_2d 点 Vec<8B>（`-0x228`，计数 `-0x258`）。
   - `func_0x00014157f370(&uStack_130,&lStack_228)` = **utils::curve 段迭代器**（每调返回一段 start/end + 标志）。逐段求**点到线段最近点**：`t = dot(p-a, b-a)/|b-a|²`，`|b-a|²` 下限 eps `_UNK_14295a0d8`(0x322bcc77≈2.6e-39)，`t = clamp(t, 0, 1.0)`（`_UNK_142925904`=1.0f）；距离开方跟踪最小值（初值 `_UNK_142925980`=f32::MAX 0x7f7fffff）。
   - t 越界 [0,1] → **panic `assertion failed: (0.0..=1.0).contains(&t)`**（`FUN_1428d9430(&UNK_142c63b78,0x2a,&UNK_142c63ba8)`，行 0x2a=42，**utils/curve/mod.rs** 域，行 487/548）。
   - 命中最近段后取段索引 uVar26/uVar25；对 3D 曲线数组越界检查（panic 行 503/508/567/572/577–589，串 UNK_142ad9428/9440/142c63658/142c63670 均 utils::curve 域）。
   - **构建段**：raw 点对 `{pt_i, pt_{i+1}}` + 高度 `h_i,h_{i+1}` + 沿最近段的插值点（`p = lerp(a,b,t)`，`t=min-dist 段`) + 插值高度（3D 曲线点线性插值）。
   - **`FUN_140aa5dd0(uStack_140=WallSpatialHash, h_i, h_{i+1}, &{pt_i,pt_{i+1}}, snapped_a, snapped_b, h_snap_a, h_snap_b, wall_id, 0x3e8f5c29)`**（行 605–607）= add_segment，**RESAMPLING=0.28f**（0x3e8f5c29）传给 get_cells_from_segment。
4. 循环至点对耗尽。

**OnWallDeleted 路径**（行 418–430）：逐 16B 事件行（param_2 事件存储），每行 **`FUN_140aa5360(uStack_140)` = remove_wall_id**。

**边界/早退**：事件为空直接返回；`cStack_f1`（puffin 开启标志）控制尾事件刷新；索引越界 panic 由 `FUN_1428d9518`（fmt）+ `invalidInstructionException` 落盘。

**常量**：`_UNK_142925980`=f32::MAX(0x7f7fffff)、`_UNK_142925904`=1.0f、`_UNK_14295a0d8`=0x322bcc77（eps）、`_UNK_142925870`=0.5f（行 234–235 加载）。

**遗留 U**：全局调试计数 `0x14362c3a8` 的语义（仅 >1 时才 panic 缺墙事件）；`func_0x00014157f370` 内部实现（utils::curve 域，仅观察接口）。
