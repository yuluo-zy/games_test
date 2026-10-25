# restorer-collision-CLOSE — collision 库内剩余项收口

日期: 2026-10-17
角色: restorer（只读证据 ghidra `--worker shared`）
协议: planning/2026-10-16-logic-verify-protocol.md、2026-10-17-recovery-sop.md
证据: planning/impl-collision-close/results/
- a.result.json（fresh：get_collider_mesh / get_rapier_collider / raycast_simple_exclude_wall_id /
  legacy_spherecast / SubworldColliderHandle::new / mesh_intersect_collider_diff_publisher /
  rapier_hit_to_raycast_world_result / spherecast）
- b.result.json（fresh：new_indexed 0x1408cee40 / new_unindexed 0x1408cf7d0 /
  construct_arch_geo 0x1408d7ed0 / upsert_collider_entry 0x140b40b50）
- disasm-sphere.result.json（legacy_spherecast 0x1408ccc90、spherecast 0x1408ccfc0 全量指令）
- disasm-mesh.result.json（new_indexed/new_unindexed 全量指令）
- disasm-arch.result.json（construct_arch_geo 240 指令）
- const.result.json（0x142aa3ec0 / 0x142aef03c read_memory）

## 1. find_mesh_mesh_intersections closure 语义（三角/merge/brick 之外）

fresh `intersect_newly_added_colliders::closure_env$0` 0x1417d1440（1071 B，复用
impl-collision-remainder2/r5）逐语句落地为
`intersect_meshes::intersect_newly_added_collider`：

1. 去重集 `HashSet<SubworldColliderHandle>` 探针命中候选 → 直接返回（集合插入点属装配侧）；
2. `should_intersect_collider_meshes`（kind 组合门控，非法组合 panic）；
3. 两侧 `as_trimesh`（ShapeType 判别 5）门控（由调用方 `&TriMesh` 承载）；
4. `intersect_mesh_mesh` → `merge_segments_into_polylines`；
5. 合并结果为空 → 返回；
6. 两侧注册表 `generation|1` 与 `kind & 0xfd == 0` 校验（失败 panic）；
7. push **0x40 B** `MeshIntersectionArtifacts`。

`MeshIntersectionArtifacts` 精确布局改为 `{Vec<Polyline> @0, mesh_ref_a @0x18,
mesh_ref_b @0x20, a @0x28, b @0x34}`（64 B 单测锁定）；删除原未锚定的
`bird_nest_candidates` 字段（fresh 0x40 B push 中不存在）。

**登记（不重复定义）**：`BrickTransformSsbo` 铺砖（geometry::instanced_wall，另一 agent
锚定中，0x60 B + SparseSsbo）；图 `add_node`×2/`add_edge` 接线；`spec_extend` 的
`closure_env$3` 鸟巢候选来源；`SparkleSsbo` allocation drop。

## 2. registry free-list / generation-bump / ColliderMeshInstance（fresh 锚定）

`dispose_collider_mesh_instance` 0x1408d9590 disasm 逐条：

- 注册表 Vec：`world+0x08` data / `+0x10` len，条目 **stride 0x28**；
- `ColliderRegistryEntry` 重锚：`first u32@0`（活=kind 字节 / 释放=free-list next）、
  `meta [u8;0x1c]@0x04`、`generation@0x20`、`generation_low@0x24`（0x28 B）；
- dispose 顺序：`ColliderSet::remove` → 校验 `index<len && entry.gen==instance.gen`
  （失败 unwrap_failed）→ `entry.first4 = world+0x18`、`world+0x18 = index`、
  `world+0x1c -= 1` → `entry.gen = gen+1` → kind==0x11 panic → 槽脏/AABB/发布；
- `ColliderMeshInstance` 重锚为 **0x18 B**：
  `{generation@0, registry_index@4, collider.index@8, collider.generation@0xc, slot@0x10,
  pad@0x14}`；`handle()` 返回注册表 (index, generation, slot)；
- `upsert_collider_entry<Entity,(Vec3,Quat)>` 0x140b40b50 fresh 锚定：source→slot 静态表
  @0x142aef03c；`IsoTransform::from_translation_rotation`+`nested_in`；Arc 克隆 @mesh+0x30；
  实时 collider 姿态比较（+0x44/+0x48/+0x4c、+0x34..+0x40，NaN 拒绝）→ Kept；否则
  dispose + SlotMap insert + build + AABB 栅格化 + `ColliderSet::insert` + publish；
  hashbrown entry stride 0x20（key Entity@0 + value 0x18）。

**登记**：free-list 空表哨兵未实测（Rust 用 `u32::MAX`）；分配侧弹栈/注册表 kind 写入点未锚定
（upsert 以 kind=2 建模）；注册表 +0x08 → mesh 源（`MeshInstanceSources`）链接未锚定。

## 3. barycentric / 法线修正（collision 域）

`rapier_hit_to_raycast_world_result` 0x1408ce4a0 fresh 逐语句修正：

- 输入 mesh 布局改为真实字段：vertices @+0x00/+0x08、indices @+0x10/+0x18（stride 0xc）、
  normals @+0x30/+0x38、uv @+0x40/+0x48（旧实现 uv 错位 +0x48/+0x50）；
- 门控：indices 空 → 非三角路径；否则 `hit 判别 != 2` → 判别 2（miss）；
- 三角下标 `tri -= (tri>=len ? len : 0)` 后仍越界 panic（O：两次 panic_bounds_check）；
- barycentric 权重 `barycentric(pos, v1, v2, v0)` 施加到法线/uv（逐条判等）；
- **法线经四元数旋转** `q * n`（fresh in_R9）；uv 不旋转；
- 法线空 → 命中法线 fallback（世界空间，不旋转）；
- 有 uv → 判别 1；无 uv → 判别 0、extra={bc.y,bc.y}。

`spherecast` 0x1408ccfc0 fresh 修正：position = `hit.witness1`（非 origin+dir*toi）、
normal = `hit.normal1`，按 `(origin - witness1)·normal1 < 0` **整体翻转**（XMM14/XMM15 掩码序列）。

## 4. new_indexed / new_unindexed 重锚

fresh store 序列（含 const read_memory）确认 `RaycastMeshData` **0x78 B**：
`+0x00 顶点 ptr、+0x08 顶点 len、+0x10 索引 ptr、+0x18 索引 len、+0x20 shape、
+0x30 法线 ptr/len、+0x40 uv ptr/len、+0x50/+0x58/+0x60=0、+0x68=1.0、+0x70 hash`
（无 cap 槽；旧实现的 vertices_cap/normals_cap/uvs_cap 与 uv 偏移均错，已改并 `repr(C)`）。

- `new_indexed(indices, vertices, Option<normals>, Option<uvs>)`：哈希 `(indices, vertices)`
  后 **Some 时**依次折叠 normals/uvs；空白切片哨兵 4；缓冲复制为独立堆后 ptr+len 存储
  （Drop 释放）；`trimesh` 用 float 顶点（`p.x as f32` → 真实 f32 读取，旧实现位型→数值转换错）。
- `new_unindexed(triangles, 逐三角 3 角点法线, Option<逐三角 3 uv>)`：顺序索引、hash=
  `(triangles 位型, 角点数组)` 再折叠 uv 数组；`+0x30` = 逐角点法线 flat；`+0x18`=三角数。

## 5. 其余 world.rs 可落地项

| VA | fresh 结论 | 处置 |
|---|---|---|
| 0x1408cdbd0 get_collider_mesh | 仅 bounds + 精确 generation + data 非空校验 | 判等 |
| 0x1408ce440 get_rapier_collider | `get_mut_internal` | 改 `&mut self`/`get_mut` |
| 0x1408ccc50 raycast_simple_exclude_wall_id | 纯转发 raycast_simple | 判等 |
| 0x1408ccc90 legacy_spherecast | **无 mask**，固定 10 槽表 `[0,1,3,7,6,4,0xb,2,0xc,0xd]` | 已修（旧 17 槽 mask 错） |
| 0x1408ccfc0 spherecast | witness1/normal1 + 点积翻转 | 已修 |
| 0x1408cea20 subworld | 静态表 0x142aef03c `[0,9,1,2,8,14,3,7,6,10,4,5,11,12,13,15,16]` | 已修 + from_slot 取逆 |
| 0x1408d7ed0 construct_arch_geo | fresh+240 指令已解：`min(f32_len,Vec3_len)` 窗口、`collect_from_iter_no_buf<Chain<Once,Zip>>`、每窗 2×`Triangle::new`+`normal`、4×`[Vec3;3]` push 到两个 Vec | **未实现-登记**（寄存器别名使逐顶点 operand 分配不可锚定，不做近似） |

## 6. obvhs

保持登记（未入 workspace），本轮未重试。

## 编译 / 测试

- `cargo check -p country-core --lib`（default）：**0 error**
- `cargo check -p country-core --lib --features native-physics`：**0 error**
- `cargo test -p country-core --lib -- systems::collision resources::rt`：**23 PASS**
- 同命令 `--features native-physics`：**38 PASS**（新增 layout/dispose/closure 4 项 + 既有）
- 全库：default **2069 passed / 0 failed**；native-physics **2084 passed / 0 failed**
  （高于协调方 2024 基线；期间一次性 ENOSPC/他组 prefab.rs WIP 错误已自愈，非本模块）
