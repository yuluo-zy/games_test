# systems/wall/wall_splitter.rs

> 文件作用：墙分裂系统：光标拖过墙到 get_curve_segment_from_u/get_u_at_coord 求 split u 到 ChildWall::new 两子墙到 InnerWalls::new_wall；命中处 WallStyle::convert_to_stone_version；装饰经归档资源重挂（split_wall/split_wall_scissors signifier）。

函数总计 3：业务 1 / 内联无独立符号 0 / 派生胶水 2

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::wall::wall_splitter::wall_splitter` | 0x140a74c20 | 0 | wall_splitter（get_curve_segment_from_u/get_u_at_coord + ChildWall::new + convert_to_stone_version） | O | wall\decomp\decomp-0x140a74c20.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 2 项

## 实现方式（算法级）

### wall_splitter 0x140a74c20（O，结构级；~14 参大系统）

签名要点：param_1 = 模式资源（**首字节 != 1 → return**，分裂模式门控）；param_2 = &PublicWalls（0x2a8 hashbrown 遍历候选）；param_4 = *param_4（UiState 查找入参）；param_5/6/7/8/9/10/11/12/13/14 = 多资源（tick 回写多处 = 多 ResMut：InnerWalls、WallEraserArchivist、DecoratorStorage、事件流、History 等）。

**分裂点计算（split u）**：
1. `UiState::add<String>`（0x1409231f0）→ UI 状态记录（+0x18 flag、+0x28 int、+0x38 ptr、+0x4c/+0x54 u32）。门控：`flag==0 || ptr!=期望 || tag==2` → 跳过。
2. `Curve::get_approx_coord_from_pos<Vec3,Thresholded3D>`（0x141469e00，光标→{段 idx, t}）；越界检查（curve/mod.rs panic_bounds_check ×2）、`t∉[0,1]` panic（curve_u.rs:42）。
3. `u = 0.5f / fVar41`（0x142925870 常量；语义 U，疑为半宽/长度比）→ clamp [0,1]（越界 panic_fmt）、`!u.is_nan()` panic（curve_u.rs:29）、`u2 = 1-u`（另一子墙区间，再 clamp）。
4. **切割点 3D 位置**：`lerp(point[u], point[u+1], t)`（12B Vec3）。
5. **单位切线**：`1/sqrt(dx²+dy²+dz²) × delta`；`utils::xz_projective_rotation_bad`（0x140c911d0）。
6. **signifier**：`split_wall_`/`split_wall`/`split_wall_scissors`（0x142ad6530/598/600）→ format_inner（0x140d79f90）+ `AppendOnlyVec::push<UiAppendSignifier>`（0x14146c880）×2；Vec3::fmt（0x140d4ba00）用于 panic 显示。

**两子墙生成（loop ×2）**：
- `Curve::crashy_slice<Vec2,2D>`（0x141466bb0）切父曲线。
- 逐点 `TerrainHeightsData::sample_at_world_xz`（0x1409e9640）求地形高度（min 追踪）。
- 组 ChildWall 载荷（点阵、高度、flags）→ `hashbrown::HashMap::insert<WallId,WallState>`（0x141529bb0，经 +0x30 工厂）+ `HashMap::insert<WallId,()>`（0x1402ca2f0，wall_ids）+ **InnerWalls +0xc0 脏代 +1**。
- 记录 {child_wall_id, u} 对（0x20B 缓冲）。
- `InnerWalls::remove`（0x14085aa60）移除原墙。

**墙型 switch（InnerWalls 条目 style 字节，clamp 0..3）**：
- case 1：`WallEraserArchivist::new`（0x14092e6e0）→ `archive_current_decorators_in_storage`（0x1413e7230，puffin 域 "wall splitter conservative"）→ `decorator_cull_allow_any`（0x140b2bc00）→ `cache_if_original_wall`（0x14092ea50）。
- case 3：进入子墙创建路径。
- default：跳过。

**收尾**：`History::add`（0x140a84ec0 ×2，子墙 style 记录，载荷 [3]=3/9 型字节 + wall_id + u32@+0x2c）→ `WallEraserArchivist::add_split_event`（0x14092f280）→ 事件行 0x18B（{u32@0, u64@+0x10}）+ 每子墙 0x18B（{seq, child_id, u32, 0}）→ `DecoratorStorage::on_wall_split`（0x140a1f590）→ `WallEraserArchivist::finish`（0x14092f990，21KB）。

**11 个 FUN_1428d9518 = `core::panicking::panic_bounds_check`（下标越界）**，非 11 条独立消息：
- 5 处 file = utils/curve/mod.rs（0x142c635d8/f0/658/670/688）：曲线取点/切片越界。
- 6 处 file = wall_splitter.rs（0x142ad6570 ×2、0x142ad66a0、0x142ad66b8、0x142ad64d8、0x142ad64f0 域）：数组访问越界。

其他 panic：`alloc::raw_vec::handle_error`（0x1428d8fe3）、`handle_alloc_error`（0x1428d9000）、`unwrap_failed`（0x1428d9310）、`panic_fmt`（0x1428d9390）、`panic`（0x1428d9430）。

### 遗留 U
- `0.5/fVar41` 的 fVar41 精确来源（切割参数归一化公式）。
- ChildWall::new 载荷中 flags（uStack_220、cStack_21f = 5×调试开关 cRam0000000143634d90、uStack_250=2）语义。
- 事件 0x18B 行的具体事件名。
