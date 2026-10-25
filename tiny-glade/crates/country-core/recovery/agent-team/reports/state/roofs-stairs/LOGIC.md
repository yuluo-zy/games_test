# LOGIC — resources/roofs + resources/stairs + systems/history/replay_stairs（逐函数逻辑判等）

> logic-roofsstairs，2026-10-17。协议：planning/2026-10-16-logic-verify-protocol.md。
> 方法：Ghidra `--worker shared` fresh 反编译 + capstone 机器码核对（RIP-relative 常量/掩码逐地址读）。
> verdict：判等 / 未实现-需重做（附分歧点）/ 胶水-非业务。
> 只以 fresh 反编译为准；未判等一律标未实现。

## 总览

| 模块 | 已审计业务 | 判等 | 未实现-需重做 | 主要分歧 |
|---|---|---|---|---|
| roofs/roof_noise.rs | 8 | 4（hash1/hash_combine2/uint_to_u01/lerp 单体） | 4（noise/hash2/apply/revert） | **noise hash 链与机器码不同构** |
| roofs/roof_visual_shape.rs | 6 | 4 | 2（height 系依赖 noise） | get_height_01 依赖错误 noise |
| roofs/roof_ridge.rs | 7 | 6 | 1（get_inner 地址错标） | get_inner 注释地址=minidump 函数 |
| roofs/roof_shape.rs | ~17 | 13 | 3（intersection_y y==floor、uv 族、clear 映射） | y==floor 以 t=0 截面近似 |
| stairs/stairs_state.rs | ~19 | 17 | 0（2 处小分歧登记） | remove_node/restore_backup 脏集/backed_up |
| stairs/structural_graph.rs | 7 | 6 | 1（clear 地址错标+connections 端点） | clear=MagicSupports 地址 |
| stairs/visual.rs | 4 | 1 | 3 | MagicSupports/clear/reset_edge 结构不同 |
| stairs/invalidate.rs | 1 | 1 | 0 | 碰撞/RT 撤除=胶水登记 |
| stairs/stairs_removed_supports.rs | 1 | 0 | 1 | 数据模型=边键集合 vs edge1→edge2 |
| stairs_walls.rs | 1 | 1 | 0 | — |
| systems/history/replay_stairs.rs | 1 | 0 | 1 | 双路径 undo 语义相反+边键模型 |
| recovered_building/wall_platform_stairs.rs | 5 | 0 | 5（无 fresh 地址，共享重建域） | 共享恢复，未单独判等 |

## 重点分歧（fresh 反编译 + 机器码证据）

### 1. roof_noise::noise（0x140a152c0）——hash 链不同构【未实现-需重做】
- 机器码每角哈希：`finalizer( combine_partial(hash1(z_corner), x_corner) + seed )`，其中
  `combine_partial = ((h>>11^h)*0x8001 + x*0x19660d)*0x19660d`，`seed` 来自角种子表
  @0x142ac7e10 = {0x0ae135d3, 0x0ae135d3, 0, 0}（ADD 到 partial 后）；
  finalizer = `(h>>11)^h → (h<<7 & 0x9d2c5680)^h → (h<<15 & 0xefc60000)^h → (h&0x7fffff)^(h>>18)`。
- Rust hash2 = `hash_combine2(hash1(x ^ (y << 8)) ^ corner, y)`：输入 `hash1(x^(y<<8))` 与角种子
  **异或进 h 在 combine 之前**，且**完全无 finalizer**。机器码里既无 `x^(y<<8)` 也无 combine 前异或，
  且多了 finalizer（掩码 0x9d2c5680/0xefc60000/0x7fffff 均在 @0x142ac7e20/30/40 实测）。
- 数值对照（/tools/venv capstone+python）：机器 (0,0) 角 u01=0.5488 vs Rust 0.7193，差异最大 0.67。
- hash1（`(v*0x401)>>6 ^ v*0x401)*9`）、hash_combine2 单体（`(h>>11^h)*0x8001 → (y*M+h+M)*M + 0xae135d3`）、
  uint_to_u01_float（`(bits & 0x7fffff | 0x3f800000) - 1.0`，掩码 @0x142ac7e40/OR @0x14292e6b0/减 @0x14292e6d0）、
  lerp 均与机器码**判等**。问题在 hash2 的组合方式与 finalizer。
- 传导：apply_roof_noise（0x1412348f0，clamp/0.3/0.5/smoothstep 结构判等）、
  get_height_01_unclamped_from_ws（0x1408e4790，自身公式/常量/断言逐指令判等，但调 noise）→
  **功能上不等价**，标未实现-需重做（分歧点=依赖错误 noise 输出）。

### 2. roof_shape_at_intersection_y（0x1408e3820）y==floor 分支【未实现-需重做】
- 机器码：`y==floor` 直接拷 `roof[8..0x24]` 28 字节原始帧体；Rust 用 `surface::section(b, 0.0)`
  （t=0 截面，末 8 字节=计算出的 bottom 尺寸，≠ roof[0x1c..0x24] 原始 dims）。源码注释自认"以 t=0 截面近似"。
- 越界分支（`y<floor || cap+floor<=y → OutOfBounds`）、cap 计算与其余分支判等。

### 3. roof_uv_at_ws_pos / roof_uv_and_point_at_ws_pos（0x1408e4040/4090）【未实现-需重做】
- 原函数是碰撞注册表（hash_one GlobalId 链→arena→raycast→barycentric）全管线（0x1408e4090 大函数）；
  Rust 是 `#[cfg(feature="native-physics")]` 直连 raycast 桩，未实现注册表/重心插值。RESTORE 已登记 R1 待接线。

### 4. StairsState::remove_node / restore_backup 小分歧【判等+登记】
- remove_node（0x1408ffef0）：机器只在**节点未找到**时插自环脏边（(node,node)）；找到时只插邻边脏边。
  Rust 恒插自环。已存在节点的脏集内容略不同。
- restore_backup（0x1408ff210）：机器不写 backed_up（保持原值）；Rust 强制 true。实践中 make_backup 后
  backed_up 已 true，差异影响低。

### 5. structural_graph::clear 地址错标 + MagicSupports 结构【未实现-需重做】
- 0x140869a30 fresh 反编译= `visual::MagicSupports::clear`（Arc downgrade/LOCK/mesh atlas 控制块），
  **不是** StairsStructuralGraph::clear。RESTORE/source 把该地址给了 structural_graph::clear（清 5 表），
  数据映射错误；Rust clear（5 表 .clear()）无 fresh 地址可判等。
- MagicSupports 机器型是带 Arc/mesh 句柄的资源；Rust 是 `HashSet<StairGraphEdge>` + `entries.clear()`，
  类型与机器不同构。
- disconnect_node_by_collider（0x14086a4c0）：机器每邻边只往 connections 插**一个**端点；Rust 插两个。
  source 注释已标"端点之一；端点序语义 U"。
- 其余 6 函数（upsert_span/upsert_support_source_collider/add_edge_by_span/add_edge_by_collider/
  remove_node_by_collider/0x140869d20..0x14086a4c0）判等。

### 6. StairsRemovedSupports + replay_stairs【未实现-需重做】
- 机器 `StairsRemovedSupports` = `HashMap<StairGraphEdge, ()>`（全边键集合）；Rust = `HashMap<u32,u32>`
  （edge1→edge2，erase 只按 edge1）。数据模型不同构（replay_stairs 0x140af75b0 的 insert/remove_entry
  key 是 StairGraphEdge 对）。
- replay_stairs 双路径：flags bit0（+8）区分两条路径，**undo 语义相反**——
  动画路径（bit0=1）：undo==0→insert、undo!=0→remove_entry（=Rust）；instant 路径（bit0=0）反之。
  Rust 单路径恒 insert-on-redo/remove-on-undo，只对动画路径判等。
- 其余结构判等：tag==10（'\n'）命令过滤、动画门控 `elapsed<=max(duration/speed, 0.05)`→advance、
  DecoratorStorage::lookup + 类型字节 `|0xf8>0xf9` 过滤、audio 事件写、done 命令逆序 memmove 移除。

### 7. visual remove_edge/reset_edge
- remove_edge（0x140aa8620）：存储核心判等（删边 + 脏字节 @+0x30 置 1）；SparseInstanceBufferAllocation
  drop 与 MagicSupports drop = render/rt 胶水（登记）。
- reset_edge（0x140aa8a60，2099B）：机器以 SparseInstanceBufferWriter 渲染写为主，Rust 只建模
  `edges.insert(edge, default)+dirty`，结构不同构 → 未实现-需重做（渲染写面未建模）。

### 8. invalidate_dirty_stair_artifacts（0x140b37180）
- 核心判等：脏边 drain（StairsState +0x148）→ 每边 `contains_edge_or_stub` → 在→reset、不在→remove_edge。
  remove_collider×3 + Arc drop + RtMesh 撤除 = collision/rt 胶水（登记）。

## 判等清单（逐函数）

### roofs/roof_noise.rs（判等 4）
- hash1 ✓ / hash_combine2（单体）✓ / uint_to_u01_float ✓ / lerp ✓

### roofs/roof_visual_shape.rs（判等 4 / 未实现 2）
- get_roof_expanded_top_and_bottom（0x140a880c0）✓（rect bottom=eave_ws+0.28+dim、top ridge_dir 选轴/0.1；
  圆 bottom=((eave_ws·1.2+0.28)·0.5+radius)、top=0.2；tag 0/1）✓
- as_circle（0x140a88200）✓（tag bit0==0→拷两 f32；1→panic）
- as_rectangle（0x140a88240）✓（bit0==1→拷 bottom@+4/top@+0xc；0→panic）
- get_roof_shape（0x1408e4920）✓（surface::section 逐字节对照判等：q=1.5p+(1-p)0.5、blend=t^(1/q)、
  矩 28B/圆 20B 布局一致）
- get_height_01_unclamped_from_ws（0x1408e4790）✗（自身公式+常量全部机器码判等：0.5/0.1/-0.3/8.0/9.0/12.75/
  0.4/12.0/3.0/-2.25/7.1/-0.9、cap=min(cap_base,max(0,cap_h))、assert、inverse_lerp 尾调；**因调错误 noise 判未实现**）
- get_roof_shape_from_pos_ws_clamped（0x1408e4750）✗（薄壳 clamp→get_roof_shape，继承 noise 分歧）

### roofs/roof_ridge.rs（判等 6 / 未实现 1）
- from（0x1408e2690）✓ / calc（0x1408e2700）✓ / inverse_ridge_01_param（0x1408e30f0）✓
  （ridge_dir 选轴、(len−0.1)/(dim−0.1)、dim==0.1→Err、NaN→Err、clamp[0,1]）
- to_world_space（0x1408e2750）✓（max≤0.1→Point、圆→Err、矩→Segment+0×height 保号零；委托共享 ridge）
- get_inner ✗（注释地址 0x1400885d0 = `minidump_unwind get_instruction`，数据错标；函数本身=首字段拷贝）
- max_dim（内联）✓（SSE max，NaN→b）

### roofs/roof_shape.rs（判等 13 / 未实现 3）
- new（0x1408e31c0）✓ / profile_curve_normalized（0x1408e3420）✓ / roof_tip_offset_xz（0x1408e3780）✓
- rotation_yaw_radians（0x1408e3b10）✓（矩 atan2(−frame.z,frame.x)；圆 CircularF32−π/2）
- ridge_length_01_from_rectangle_ratio（0x1408e3b60）✓（机器码：ratio=max(w/h,h/w)、u=clamp((max_dim−2.25)/5.25)、
  smooth=(3−2u)u²、blend=(1.5·smooth+ratio−1.5)/1.5、clamp+smoothstep；常量 5.25/1.5/−1.5 实测）
- roof_height_ws_from_wall_shape（0x1408e3c70）✓（机器码：dim_n、SineInOut@0x142aa5938、
  low/high=10/4·6/3（@0x1429345a8/694/690 实测）、clamp[0,12]）
- roof_height_01_from_wall_shape ✓（内联 /12 clamp[0,1]）
- eave_length_ws（0x1408e3c30）✓（公式在 get_roof_shape decompile 中复核）
- height_ws ✓（h·12+(1−h)·0.4）/ profile_curve_ws（0x1408e3d40）✓（u·top+(1−u)·bottom、v·height、Curve2）
- is_wooden（0x1408e31a0）✓（tag 0→other、1→false、2→byte@+1）/ pattern_id ✓（byte@+1）
- clone_behavior（0x1408e6af0）✓（None）
- GableRoofStyle::is_stepped/is_overhang ✓（tag==0/tag!=0；但 source 地址错标：0x140833230=
  `ClutterVersion::is_initial`、0x1408c5680=`StructureType::is_fence`，非本函数）
- roof_shape_at_intersection_y（0x1408e3820）✗（y==floor 近似）/ roof_uv_at_ws_pos ✗ / roof_uv_and_point_at_ws_pos ✗

### stairs/stairs_state.rs（判等 17 / 小分歧 2 登记）
- 判等：add_node(0x1408ffd30)/add_edge(0x1408ffd70)/disconnect_edge(0x1408ffe80)/try_set_endpoint
  (0x1408fe7b0)/endpoint/unset_endpoint/get_all_connected_nodes(0x1408fe8c0)/make_backup(0x1408fefe0)/
  compute_checksum(0x1408ff430)/restore_snapshot(0x1408ff590)/extract_diff_and_make_backup(0x140900010)/
  reduce_diff(0x1409007b0)/apply_diff(0x140900ba0)/StairsSnapshot::new(0x140901120)/get_or_allocate
  (0x1408fe5e0)/contains_edge_or_stub(0x140900e80)/contains_edge_or_stub_impl(0x140900fd0)
- remove_node（0x1408ffef0）✓+登记自环脏边差异 / restore_backup（0x1408ff210）✓+登记 backed_up 差异

### stairs/structural_graph.rs（判等 6 / 未实现 2）
- 判等：upsert_span/upsert_support_source_collider/add_edge_by_span/add_edge_by_collider/
  remove_node_by_collider
- 未实现：clear（地址错标=MagicSupports::clear）、disconnect_node_by_collider（connections 单端点 vs 双端点）

### stairs/visual.rs（判等 1 / 未实现 3）
- remove_edge（0x140aa8620）✓（存储核心 + 脏字节 @+0x30；渲染胶水登记）
- MagicSupports::clear（0x140aa8210 / 0x140869a30）✗ / reset_edge（0x140aa8a60）✗ / force_rebuild ✗（=reset_edge）

### stairs/stairs_removed_supports.rs（未实现 1）
- record/erase/contains：数据模型（边键集合 vs edge1→edge2）不同构 ✗（serde 胶水不计）

### stairs/invalidate.rs（判等 1）
- invalidate_dirty_stair_artifacts（0x140b37180）✓（核心；碰撞/RT 撤除胶水登记）

### stairs_walls.rs（判等 1）
- StairsPillarProposals::add（0x140901270）✓（hashmap key→44B 桶 append）

### systems/history/replay_stairs.rs（未实现 1）
- replay_stairs（0x140af75b0）✗（结构判等部分见上；分歧=双路径 undo 语义 + 边键模型）

### recovered_building/wall_platform_stairs.rs（未判等 5）
- collect_platform_stair_blockers/prepare_platform_stair_segments/platform_stair_cell/
  platform_stair_geometry/platform_stair_holes：无 inventory original_functions、无 fresh 地址映射，
  属共享 recovered_building 重建域（自带验证证据），本 pass 无法逐函数判等，标未判等-待共享域。

## 纪律执行
- 未判等函数均标未实现-需重做（不采信"待取证登记"通过）。
- 全部以 fresh 反编译为准；常量/掩码经 read_memory 实测。
- 无修改 src；无写库操作。
