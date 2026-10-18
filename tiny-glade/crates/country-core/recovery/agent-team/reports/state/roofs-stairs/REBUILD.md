# REBUILD — roofs+stairs 17 个需重做函数：分类 + 真实控制流重建

> restorer-rebuild-roofsstairs，2026-10-17。协议：planning/2026-10-16-logic-verify-protocol.md
> （含宏生成函数分类规则）；判等基线：reports/state/roofs-stairs/LOGIC.md。
> 方法：Ghidra `--worker shared` fresh 反编译（fields=decompiled_code,callees）+ capstone
> 机器码核对（disconnect_node_by_collider 端点、noise 常量/掩码）。
> **只以 fresh 反编译为准；重建逐分支与机器码一致；判等函数（53 个）未动；未改他模块。**

## 一、分类结论（宏生成胶水 vs 真实业务）

| 函数 | 地址 | 类别 | 结论 |
|---|---|---|---|
| roof_noise::noise | 0x140a152c0 | 真实业务 | 重建（hash 链 + SIMD 双线性） |
| roof_noise::hash2 | 内联 | 真实业务 | 重建（finalizer(combine_partial(hash1(z),x)+seed)） |
| roof_noise::apply/revert_roof_noise | 0x1412348f0 | 真实业务 | 结构判等；依赖 noise，修正后自动修复（未改） |
| roof_visual_shape::get_height_01_unclamped_from_ws | 0x1408e4790 | 真实业务 | 公式判等；依赖 noise，修正后自动修复（未改） |
| roof_visual_shape::get_roof_shape_from_pos_ws_clamped | 0x1408e4750 | 真实业务 | 薄壳 clamp；依赖 noise，修正后自动修复（未改） |
| roof_ridge::get_inner | 无 fresh 地址 | 真实业务 | 首字段拷贝实现正确；仅修正错标地址注释 |
| roof_shape::roof_shape_at_intersection_y | 0x1408e3820 | 真实业务 | 重建 y==floor 原始帧体拷贝 |
| roof_shape::roof_uv_at_ws_pos | 0x1408e4040 | 真实业务 | **R1 待接线，登记未实现** |
| roof_shape::roof_uv_and_point_at_ws_pos | 0x1408e4090 | 真实业务 | **R1 待接线，登记未实现** |
| structural_graph::clear | 0x140869a30 | 真实业务 | 地址正确（LOGIC §5 误判已驳回）；清 8 表 |
| structural_graph::disconnect_node_by_collider | 0x14086a4c0 | 真实业务 | 重建（每邻边单端点） |
| visual::MagicSupports::clear | 0x140aa8210 | **render/rt 胶水** | 类型更正（非 HashSet）+ 登记 |
| visual::reset_edge / force_rebuild | 0x140aa8a60 | 存储核心+渲染胶水 | 存储核心判等；渲染写面登记 |
| stairs_removed_supports::record/erase/contains | — | 真实业务 | 重建数据模型 = `HashMap<StairGraphEdge, ()>` |
| systems::history::replay_stairs | 0x140af75b0 | 真实业务 | 重建双路径 undo + 全边键 + 下限 0.05 |

非胶水判定：以上均无 derive/serde visitor/Bevy 注册体，全部为真实算法/数据流，按业务重建。

## 二、重建明细（fresh 反编译 → 控制流 → 逐语句）

### 1. roof_noise 0x140a152c0 — hash 链 + SIMD 双线性【重建】
机器码（capstone disasm_noise）逐指令：
- 每角哈希链 = `finalizer( combine_partial(hash1(z), x) + seed )`；掩码 0x9d2c5680/0xefc60000/0x7fffff
  （@0x142ac7e20/30/40 read_memory 实测）、OR 0x3f800000（@0x14292e6b0）、减 1.0（@0x14292e6d0）。
- **SIMD 结构**（非旧实现的 x0+1 四角）：每 z 行两角——scalar 角 = `hash_combine2(hash1(z), x)`
  = `combine_partial + 0x0ae135d3 + 0x19660d²`；simd 角 = `combine_partial(hash1(z), x) + 0x0ae135d3`
  （角种子表 @0x142ac7e10 元素 0/1）。行内 `lerp(simd, scalar, sx)`、行间 `lerp(·,·,sz)`（机器序）。
- 已删旧编造：`hash1(x^(y<<8)) ^ corner` 前置异或（机器无 `x^(y<<8)`、无 combine 前异或）；补 finalizer。
- 数值对照（同公式 f32 复刻，全过）：noise(0,0)=0.5487866（LOGIC §1 机器 (0,0) 角 0.5488）/
  (0.5,0.5)=0.4374025/(1,1)=0.2845558/(10,-3)=0.5514752 等 9 点。
- 新常量：`CORNER_SEED=0x0ae135d3`、`SCALAR_CORNER_SEED=0x2219927c(=CORNER_SEED+0x19660d² mod 2³²)`。
- 传导：`get_height_01_unclamped_from_ws`/`get_roof_shape_from_pos_ws_clamped`/`apply/revert_roof_noise`
  调用 `noise(pos.x·0.5, pos.z·0.5)` 不变，噪声输出修正后功能等价（标「依赖修正已修复」）。

### 2. roof_shape_at_intersection_y 0x1408e3820 — y==floor 原始帧体【重建】
机器码 y==floor 分支逐 8 字节拷贝：`out[0..]=roof[8..]`、`out[8..]=roof[0x10..]`、
`out[0xc..]=roof[0x14..]`、`out[0x14..]=roof[0x1c..]` → 覆盖 `out[0..0x1c]=roof[8..0x24]` 28B 原始帧体。
- 旧实现 `surface::section(b, 0.0)`（t=0 截面近似）已删：t=0 截面末 8 字节=计算出的 bottom 尺寸，
  ≠ 原始 dims（`roof[0x1c..0x24]`）。
- 重建：tag==1 → `Rectangle{frame: roof[0xc..0x18]×4, dimensions: roof[0x1c..0x20]×2}`；
  tag==0 → `Circle{frame: roof[0xc..0x10]×2, radius: roof[0x14], word_0x18: roof[0x18..0x1c]}`。
- 其余分支（越界 `y<floor||cap+floor<=y`→OutOfBounds、`t=(y−floor)/cap` 截面）判等未动。

### 3. structural_graph::clear 0x140869a30 — 地址判定更正【保持】
- **LOGIC §5 误判驳回**：fresh 反编译 0x140869a30 = `StairsStructuralGraph::clear`（8 张 hashbrown
  表 @+0x18/+0x60/+0x90/+0xc0/+0xf0/+0x120/+0x150/+0x180 的 memset+计数归零），**非** MagicSupports::clear。
  （LOGIC 采信体 `clear.c` 实为 0x140aa8210 的 Arc downgrade 体，地址混标。）
- Rust clear 清 6 个建模集合（petgraph 图 + 5 映射）与机器 8 表同语义；+0x60/+0x120/+0x180 三张
  U 表未建模，登记（语义沿用 RESTORE §四.2）。

### 4. disconnect_node_by_collider 0x14086a4c0 — 单端点【重建】
机器码（capstone）：每邻边**单次** `HashMap::insert<handle,()>`，键 = petgraph `edges(node)` 输出
首端点 = **查询节点（被断开 collider）**（petgraph 0.8.1 `Edges::next` = `(a=self.from, b)` 证实）。
- 旧实现插两端点已更正：只 `connections.insert(collider)`。
- 登记：边权重 `StructuralGraphConnection` 未建模（`()` 恒过机器 `*weight!=0` 判别）。

### 5. StairsRemovedSupports — 全边键集合【重建】
机器码 replay_stairs 0x140af75b0 增删 key = `StairGraphEdge`（`HashMap::insert<StairGraphEdge,()>` /
`remove_entry<StairGraphEdge,()>`），值 `()`。
- 旧 `HashMap<u32,u32>`（edge1→edge2、erase 只按 edge1）不同构，已改 `HashMap<StairGraphEdge, ()>`。
- `record(edge)` / `erase(&edge)` / `contains(&edge)`。StairGraphEdge 补 serde derive（保存域 wire 格式 U）。

### 6. replay_stairs 0x140af75b0 — 双路径 undo【重建】
机器码：命令 flags bit0（@+8）分两路径——
- **动画路径（bit0==1，本模型 `instant==false`）**：门控 `elapsed<=max(duration/speed, 0.05)` 先判后加；
  就绪 → undo==0 → insert、undo!=0 → remove_entry（= 旧 Rust 语义）。
- **instant 路径（bit0==0，本模型 `instant==true`）**：立即处理；undo 语义**相反**
  （undo==0 → remove_entry、undo!=0 → insert）。
- `instant` 位语义按 0x1409f1ab0 make_animations_instant（default case 清 flags byte → instant）
  定：`instant==true` = 机器 bit0==0（立即路径）。
- **登记 U**：fresh 反编译显示 undo 判定实际读**边记录标志字节**（payload+0x10 `&1`，独立于命令
  bit0）——两路径均按该字节切 undo 分支；该字节源 U，本实现按命令 instant 位映射双路径
  （与任务指令一致），payload 标志字节登记待取证。
- `REPLAY_DURATION_FLOOR` 由 0.1 改 **0.05**（机器码 `if (fVar44 <= 0.05) fVar44 = 0.05`）。
- 其余结构判等未动：tag==10 过滤、decorator lookup + `|0xf8>0xf9` 过滤、CachedDecoratorTransforms、
  音频事件（胶水 U）、完成命令逆序 memmove 移除。

### 7. visual::MagicSupports 0x140aa8210 — 类型更正【登记】
- 机器型 = render 资源 `{+0x10 计数 u64, +0x18 计数 u32, +0x20/+0x28 与 +0x30/+0x38 两个 mesh-atlas
  Arc/写锁, +0x40 标志}`；`clear` = Arc downgrade → `SparseInstanceBufferWriter::clear<BrickTransformSsbo>`
  ×2 + 计数归零，**全属 render/rt 胶水**。
- **任务指令"HashSet 与机器同构"被 fresh 反编译驳回**：机器无边集合，旧 `HashSet<StairGraphEdge>`
  为编造；改为 render 资源占位（count/count32/render），删除编造的 insert/remove/contains 等
  （库内无消费面）。

### 8. visual::reset_edge 0x140aa8a60 / force_rebuild【存储核心判等 + 渲染胶水登记】
- 机器 = edges `HashMap<StairGraphEdge, StairsEdgeVisualState>` upsert（bucket 写入）+ 渲染写面
  （SparseInstanceBufferWriter::clear ×2、MagicSupports::clear、entity despawn）——渲染面未建模登记。
- Rust `edges.insert(edge, default)+dirty` = 存储核心（upsert），判等；force_rebuild = reset_edge 语义。

### 9. roof_uv 族 0x1408e4040/4090【R1 待接线登记】
- fresh 反编译确认全管线：`roof_uv_and_point_at_ws_pos` = GlobalId 碰撞注册表（hash_one →
  arena 槽位 → 取 collider → raycast → `rapier_hit_to_raycast_world_result`/xvy/barycentric）1152B；
  `roof_uv_at_ws_pos` = 薄壳 Option 包装。
- Rust `#[cfg(native-physics)]` 直连 raycast 桩**未实现**注册表/重心插值——按任务登记 R1 待接线，
  不重建。

### 10. roof_ridge::get_inner【地址更正 + 实现判等】
- 地址 0x1400885d0 = `decorator_storage::get`（decorator_storage.rs:114 混名），非本函数；
  本函数无 fresh 地址（NotFound）。函数本身 = 首字段拷贝，当前 `Self(self.0)` 正确，注释已更正。

## 三、自检

- **编译**：`cargo check -p country-core` **通过（0 error）**；11 警告全他域
  （audio/save/ivy/finish_replay 等）。
- **单元测试**：全库 `cargo test --lib` 被 **rhapsody_contract/render 等 21 个他模块的预存在
  测试编译错误**阻断（`GraphicsDevice(1)` → `Device` 结构字面量族，E0423 等 55 处，非本模块，另一
  worker 在途）；本模块新增/改动测试无法经 cargo test 运行。已用**独立 harness**（rustc -O 复刻
  roof_noise 全部函数）核对：9 点噪声值 + hash2(0,0,seed) u01 + scalar==finalize(combine2) 全过。
- 逻辑推演：roof_shape y==floor 帧体拷贝、replay_stairs 双路径、structural_graph 单端点、
  stairs_removed_supports 边键集合——均按 fresh 反编译逐分支实现，测试断言与实现一致。
- 判等函数（53 个）未动；他模块未改（仅 stairs_state::StairGraphEdge 补 serde derive 供
  StairsRemovedSupports 序列化，非行为改动）。

## 四、待取证（U，诚实登记）

1. replay_stairs undo 判定的 payload 标志字节（payload+0x10 `&1`）源与语义。
2. structural_graph 8 表中 +0x60/+0x120/+0x180 三张 U 表语义；边权重 `StructuralGraphConnection` 字节。
3. StairGraphEdge serde wire 格式（保存域）；StairsReplayCmd::DEFAULT_DURATION 原值。
4. roof_uv 族碰撞注册表 → raycast → barycentric 全管线（R1 接线）。
5. MagicSupports/reset_edge 的 SparseInstanceBufferWriter/mesh-atlas/entity despawn render 面。
