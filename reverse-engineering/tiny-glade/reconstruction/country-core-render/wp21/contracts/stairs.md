# 合同：resources::stairs 余量（stairs_state / stairs_assembly_params / stairs_collider / structural_graph / stairs_removed_supports / invalidate + stairs_walls）

版本 B。方法同 WP21 其它族。

## stairs_state.rs（O：权威楼梯图；petgraph GraphMap + 历史/快照）

PDB 实体：`StairsState` / `StairGraphEdge` / `StairsSnapshot` / `StairsHistoryDiff` /
`TransientStairEdgeId(Allocator)` / `StairSegment`。

- **`StairGraphEdge`**：`{ a: u32, b: u32 }`（12B 键，hash_one<StairGraphEdge>；双向边
  a<b 归一化）。**`TransientStairEdgeIdAllocator`** = `HashMap<StairGraphEdge, u32>` +
  next（`get_or_allocate(a,b)` 0x8fe5e0：已有返回，否则插 id=next+1）。**`TransientStairEdgeId`** = u32。
- **`StairsState`**（~0x180B）：
  - `graph: petgraph::graphmap::GraphMap<DecoratorId, (), Undirected>`（@+0x30，节点数
    @+0x38）—— 楼梯结构图，节点=DecoratorId（墙/楼梯节点）。
  - endpoint 状态 @+0x10（0=无/1=单节点/2=IndexMap 查找/3=unset）+ 数据 @+0x18/+0x20
    （`endpoint` getter 0x8fe890；`try_set_endpoint` 0x8fe7b0 在候选已≥8 邻边时置 3；
    `unset_endpoint` 0x8fe880 置 3）。
  - 邻接 IndexMap<DecoratorId, Vec<(DecoratorId, CompactDirection)>>、边簿记 IndexMap、
    脏标志（+0x2f 字节）、dirty edges set、备份区（+0xb8 起 0x90B）、snapshot/
    diff 域（+0x120/+0x130…），详布局 U（偏移 O 已列部分）。
  - **add_node**（0x8ffd30）：GraphMap::add_node + `HashMap<StairGraphEdge,()>` 插桩。
  - **add_edge**（0x8ffd70）：自环 panic；归一化 (min,max)；endpoint 命中短路；否则
    GraphMap::add_edge + 3×边/桩 map 插。
  - **disconnect_edge**（0x8ffe80）：同端点→remove_node，否则 remove_edge；成功则插桩+清脏。
  - **remove_node**（0x8ffef0）：遍历邻边插桩 + GraphMap::remove_node。
  - **get_all_connected_nodes**（0x8fe8c0）：从端点 BFS，收集连通节点 id，
    insertion/ipnsort 排序 + 去重，返回 `Vec<DecoratorId>`。
  - **make_backup**（0x8fefe0）/ **restore_backup**（0x8ff210）：图状态 0x90B 备份/恢复；
    未备份时收集克隆；恢复清当前图再拷贝备份，置 +0x178=1。
  - **compute_checksum**（0x8ff430）：iter_edges_and_stubs → `StairsSnapshot::new` →
    `utils::calculate_hash<StairsSnapshot>`。
  - **restore_snapshot**（0x8ff590）：算 checksum、collect_edges、reduce_diff、apply_diff、
    清临时脏集合、恢复备份。
  - **extract_diff_and_make_backup**（0x900010）：快照→diff（removed/added 边对列表）。
  - **reduce_diff**（0x9007b0）/ **apply_diff**（0x900ba0）：对图应用移除/添加边对。
  - **contains_edge_or_stub(_impl)**（0x900e80/0x900fd0）：节点或边存在性（含 stub）。
  - **iter_edges_and_stubs<closure>**：按闭包遍历边+桩（3 实例化）。
- **`StairsSnapshot`**（0x901120 `new`）：`{ checksum: u64, edges: Vec<(DecoratorId,
  DecoratorId)> }`（先排序再算 checksum）。**`StairsSnapshot::edges`** getter。
- **`StairsHistoryDiff`**：`{ removed: Vec<(u64,u64)>, added: Vec<(u64,u64)> }`；`is_empty`。
- **`StairSegment`**：段记录（0x901260 `new` 极短；布局 U）。
- 复用边界：`petgraph`（graphmap，0.8.3 缓存可用）、`indexmap`（2.7.1）、`std::hash::random::
  RandomState`（std HashMap=foldhash）、`foldhash` seed。**`DecoratorId`** = u32 newtype。

## stairs_assembly_params.rs（O：装配参数族）

- **`PlayermadeStairsAssemblyParams`**：一段手工楼梯装配参数：`points: Vec<Vec2>`
  （@+0x8/+0x10）、`heights: Vec<f32>`（@+0x40/+0x48）等。`length_ws`（0x959ca0）：
  zip 点/高，xvy swizzle，收集 Vec3 链求墙空间长。
- **`StairWallAnchor`**：`{ anchor: Vec2?, angle: f32 @+0x8 }`；`new`（0x959e80）；
  `wall_normal`（0x959e90）= `(sin(angle), cos(angle))`。**`StairNodeAnchor`**：节点锚点
  （墙/自由）；`try_as_wall`（0x959ef0）→ `Option<&StairWallAnchor>`。
- **`AlignKind`**：对齐种类；`position_offset`（0x959f00）、`outgoing_dir`（0x959f50，
  判别 0/1 否则 panic）。
- **`PlayermadeStairsAssemblyRequests`**：装配请求列表（`prev_requests` 0x274480）。
- **`StairGraphEdgeIdentity`**：32B 边标识（invalidate 中作 map 键）。
- **`StairSubSegment`**（smallvec<[StairSubSegment;1]> 值）、**`StairArchWallSupportType`**、
  **`AdjustedStairEdge`**、**`SampledStairsPoints`**：装配辅助类型（U）。

## stairs_collider.rs（O：标记资源）

- **`StairColliderRegistry`**：楼梯碰撞体注册（rapier 碰撞世界边界）。无独立函数。U。

## structural_graph.rs（O：碰撞支撑图）

- **`StairsStructuralGraph`**：楼梯支撑的**碰撞体图**（与 stairs_state 图分层）：
  - `span→collider: HashMap<(StairGraphEdge, u32), SubworldColliderHandle>`（0xc0 区，
    entry 0x18B）+ 逆映射 `collider→(StairGraphEdge,u32)` + `collider→f32`（支撑高）+ visited 集合。
  - `graph: petgraph::GraphMap<SubworldColliderHandle, StructuralGraphConnection, Undirected>`。
  - `clear`（0x869a30）：清 8 张表。
  - `upsert_span`（0x869d20）：删旧 span→加节点/边/高/visited（`'\x11'`(17)=句柄空判别）。
  - `upsert_support_source_collider`（0x869f90）：只加节点/高/visited。
  - `add_edge_by_span`（0x869ff0）/ `add_edge_by_collider`（0x86a260）：GraphMap 加边 +
    `IndexMap<(h1,h2), StructuralGraphConnection>` 写 0。
  - `remove_node_by_collider`（0x86a320）/ `disconnect_node_by_collider`（0x86a4c0）：
    删邻边→remove_node / 枚举边 remove_edge（迭代器 0x18B 步长）。
- **`StructuralGraphConnection`**：边权（u8 判别 0=…，O）。**`SubworldColliderHandle`** =
  碰撞世界句柄（collision::world，枚举，17=空）。复用：petgraph/indexmap/hashbrown。

## stairs_removed_supports.rs（O：serde 存档类型）

- **`StairsRemovedSupports`**：已被移除的楼梯支撑集（存档序列化，serde derive；
  serialize/deserialize 各 1 编译实例）。字段 U（判别枚举）。

## invalidate.rs（O：系统面）

- **`invalidate_dirty_stair_artifacts`**（0xb37180，3455B）：ECS 系统。脏边 Drain→收集；
  对每条脏边：RaycastWorld::remove_collider×3、drop Arc<RaycastMeshData>、RtWorld 删
  RtMesh（DirtyCow）、删 `HashMap<StairGraphEdgeIdentity, SmallVec<StairSubSegment>>` 与
  `HashMap<StairGraphEdgeIdentity, StairArchWallSupportType>`、`StairsVisualState::remove_edge`，
  邻接清理；`contains_edge_or_stub` 决定是否撤视觉/RT。依赖：RaycastWorld（collision）、
  RtWorld、visual::StairsVisualState —— 复用边界；系统级接线超出本资源合同（O 语义，
  实现面 skeleton 标注）。

## stairs_walls.rs（O：StairsPillarProposals::add 0x901270）

- **`StairsPillarProposals`** = `HashMap<StairGraphEdgeIdentity, Vec<StairsPillarProposal>>`
  （extend 共享 crate WP12 facade）。
- **`StairsPillarProposal`** = 44B（0x2c）：`{ u64×4 @+0x0..0x20, u64 @+0x1c(覆盖 0x1c..0x24),
  u64 @+0x24 }` —— 柱子的位置/朝向/参数（字段名 U）。`add`：按键找 Vec，push 44B 记录。
- **`StairArchType`** / **`StairsArchSupportAssemblyParams`** / **`StairsArchSupportRequests`** /
  **`StairsBracketAssemblyParams`** / **`StairsBracketsRequests`**：楼梯柱/拱/托架装配族
  （关联类型，U）。

## 输入 / 输出 / 未知

- 输入：楼梯编辑系统（decorator_visual::stairs::maintain_stair_graph）改图；装配请求。
- 输出：结构图驱动楼梯柱/支撑 mesh 装配 + 碰撞 + RT 网格；历史 diff 进存档。
- 未知（U）：`StairsState` 0x180B 完整布局/各簿记字段名；`StairSegment` 布局；
  `StairsRemovedSupports` 判别；`StairsPillarProposal` 字段语义；`SampledStairsPoints` 等。

## 复用边界

- petgraph 0.8.3、indexmap 2.7.1、foldhash 0.1.4（原锁基线；缓存可用）→ 直接声明。
- `systems::collision::world::{RaycastWorld, SubworldColliderHandle, ColliderMeshInstance,
  RaycastMeshData}`、`visual::StairsVisualState` → 碰撞/视觉域边界（typed gap 或骨架签名）。
- smallvec（SmallVec<[StairSubSegment;1]>）→ 若缓存有则声明，否则 typed gap。
