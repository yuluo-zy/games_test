# collision-eco 域算法级深挖（2026-10-15）

> 配套：DIGEST.md（布局/符号表）+ round2/collision-eco-CLOSURE.md（卡点闭合）+ 本文件（算法级）。
> 证据：`decomp/` 反编译 + llvm-objdump 原指令 + rne-read 常量/串。O=机器码直接证实；I=推断；U=未决。
> 本轮新裁决（机器码）：restore_backup 的 +0x2f 误记、closest_segment_index 步长/过滤、legacy_spherecast 结果布局、
> ivy grower 10 相跳表字节级验证、DrawState::update 精确转移。

---

## 1. 处理链图（总览）

```
[发布者] 墙/楼梯/自动杂饰系统 ──tabloid publish──▶ DirtyTiles(64×64 位图, 0x200B)
                                                      │ 每槽 publisher @slot+0x150
                                                      ▼
[flush_rapier_colliders 系统] RaycastWorld::flush_colliders (0x1408cd5c0)
   17 槽 × (脏标志@slot+0x388? → drain 0x200B@slot+0x188 → publish<DirtyTiles> → QueryPipeline::update(slot+0x0, slot+0x30))
                                                      │
[查询面] RaycastWorld::raycast / raycast_simple / spherecast / legacy_spherecast / raycast_w_terrain
   mask 位 k = 槽 k（位 0x20 = 槽 5 Terrain）→ rapier 槽内 cast_ray(_and_get_normal) / cast_shape
   → 命中 → 槽竞技场回查 {index,gen}（0x28B entry：{handle 24B @+0, 命中视图 ptr @+0x18, gen|1 @+0x20, index @+0x24}）
   → rapier_hit_to_raycast_world_result（barycentric 插值）→ RaycastWorldResult2 0x40B
                                                      │
[写入面] upsert_collider<T> 泛型族 → upsert_collider_entry (0x140b40b50)
   同 handle+同几何 → 未变(status 2)；几何变 → dispose_collider_mesh_instance + 重建 + QueryPipeline::update
   update_rapier_colliders 系统 (0x1408d0490)：逐子世界变更网格 → parry 重建 → upsert/dispose
```

---

## 2. 碰撞世界（systems/collision/world.rs）逐函数算法

### 2.1 flush_colliders（0x1408cd5c0，系统方法）— O 全解

处理链（每帧由 `flush_rapier_colliders` 0x1408d8360 以 ResMut<RaycastWorld> 调用）：

1. **puffin scope**：`func_0x000140c16a00()` 检查 profiler 活跃；活跃则按串 `&UNK_142aa3695`（模块路径 `country_core::systems::collision::world`，0x46=70 字符）开启 scope（串 @0x142aa38b8/0x142aa38ea）。
2. **逐槽循环**（`lVar13` 步长 0x390，0 → 0x3c90，17 槽；槽基 = self+0x20）：
   - 脏标志 = `byte[self + 0x3a8 + slot*0x390]`（= slot+0x388）：
   - 非零 → 执行：
     a. **drain**：`memcpy(0x200B)` 从 `slot+0x188`（脏瓦片缓冲）到栈上局部（func_0x000142923c00）；
     b. **清零**：`memset(slot+0x188, 0, 0x200)`（func_0x000142923c20）；
     c. **publish**：`FUN_141453350(slot+0x150 的 Publisher<DirtyTiles>, 局部瓦片)` —— 每槽独立 publisher @+0x150；
     d. 清脏标志字节 = 0；
     e. **日志**：若全局帧计数器 `0x14362c3a8 > 4` → `log::debug!("Flushing colliders of {slot_idx}")`（格式串 @0x142aa3900/0x142aa3928=0x27 字符，模块 @0x142aa3910；slot 序号作参数）；
     f. **QueryPipeline::update**：`func_0x000140b944a0(slot+0x30 pipeline, slot+0x00 ColliderSet)` —— 仅脏槽重建查询管线（非脏槽跳过）。
3. 关闭 puffin scope（lStack_70 追踪是否开启）。

要点：dirty 位图缓冲（0x200B=512B=4096 位，对应 64×64 瓦片表）按槽 drain→publish→清空；查询管线只对脏槽更新。

### 2.2 Rapier::raycast（0x1408cdc60，784B）— O 全解

```
raycast(out 0x40B, world, ray, filter?, max_toi, mask u32):
  iVar12 = 4 (无命中哨兵)；lVar9 = 0x20
  for slot 0..16 (mask 位 k 驱动，跳过未选槽):
    命中? = QueryPipeline::cast_ray_and_get_normal(
        pipeline=全局 0x142aa3858, colliders=world+0x3d80, slot,
        ray, max_toi, solid=1, filter)
    if 命中 (iStack_b0 != 4): 保存 {normal/toi/feature/index, slot} —— 后命中覆盖先命中（不比较距离）
  if 无命中: out.marker@+0x34 = 2; return
  解析: puVar11 = get_collider(slot, hit.index, hit.gen)（func_0x000140b93940）
  有效性: index < world+0x10 且 arena@+8 且 entry+0x20 == gen|1，否则 panic（0x142aa3c90/0x142aa3ca8）
  rapier_hit_to_raycast_world_result(out, ray, hit{feature@+0,index@+4,toi@+8,normal@+0xc..},
                                     puVar11+0x34 (barycentric), arena_entry)
```

注意：后命中覆盖先命中 —— 槽序靠后/管线顺序靠后的命中胜出，不是最近距离。两个 pipeline 全局分工：raycast 用 0x142aa3858（带法线），raycast_simple 用 0x142aa37f8。

### 2.3 Rapier::raycast_simple（0x1408ce180，696B）— O 全解

```
raycast_simple(out, world, ray, filter(2×u64), max_toi f32, mask u32):
  同 17 槽 mask 遍历; 每槽 QueryPipeline::cast_ray(pipeline 0x142aa37f8, ..., solid=1)
  命中即保存 {index/gen, slot, toi} —— 后命中覆盖先命中
  无命中: out.byte0 = 0x11 (Empty 判别); return
  命中: 竞技场有效性检查后写小结果 0x28B:
    {entry[0..1] @+0x00 (16B), entry[2] (tag) @+0x10,   ← 24B handle
     point = origin + toi*dir @+0x18 (Vec3),
     toi @+0x24}
  —— 不写法线、不写 marker、不写 uv；toi 在 +0x24（非 +0x30）
```

`RaycastWorld::raycast_simple`（0x1408ccc20，39B）= 纯委托。**`raycast_simple_exclude_wall_id`（0x1408ccc50，57B）= 同签名尾调用**：exclude_wall_id（u128，调用方 param_4/5）被原样转发进底层 cast_ray 的 filter 槽（param_4/5），函数体本身不分支于该参数 —— **exclude 在本构建无效**（DIGEST 结论，机器码复核）。

### 2.4 RaycastWorldResult2 0x40B 填充与 marker 语义 — O 全解

`rapier_hit_to_raycast_world_result`（0x1408ce4a0）算法：

```
hit_to_result(out, ray, hit, barycentric, collider_entry):
  point = origin + toi*dir
  view  = collider_entry[+0x18]           # 命中视图指针
  if view.normals_len(+0x28) == 0:        # 网格无法线
      normal = hit.normal (几何法线 @hit+0xc)
      out = {handle = entry[0..2] 24B @+0x00, point @+0x18, normal @+0x24,
             toi @+0x30, marker @+0x34 = 0}; return
  if hit.feature != 2 (非三角形): out.marker@+0x34 = 2; return   # 只写 marker，其余陈旧
  # 三角形 feature：
  i0,i1,i2 = view.indices[hit.tri_index]（{ptr@+0x10, len@+0x18}，stride 0xc，越界 panic）
  bary = 叉积式重心坐标（(P−V0)×(V1−V0) 三轴交叉积，来自 hit 附带的 barycentric 块）
  if view.normals_ptr(+0x40) != 0:  n = bary插值(normals[i0..i2])   # stride 0xc
  else:                             n = hit.normal (几何)
  if view.uv_ptr(+0x50) != 0:
      uv = bary插值(uvs[i0..i2])（stride 8）; marker = 1
  else: uv = 0; marker = 0
  out = {handle 24B, point, n@+0x24, toi@+0x30, marker@+0x34, uv@+0x38}
```

**marker 三值语义（本轮精化）**：`0` = 命中（无 uv 插值 / 几何法线路径）、`1` = 命中且 uv 已插值、`2` = 无命中（只写 marker）。消费方必须先查 marker。命中视图（0x60B）字段：`{+0x10 indices ptr, +0x18 indices len, +0x28 normals len(存在检查), +0x40 normals ptr, +0x48 normals len, +0x50 uvs ptr, +0x58 uvs len}`（CLOSURE 的 ±8 偏移差异由此表收口 —— 读取面是命中视图而非 RaycastMeshData）。

### 2.5 RaycastWorld::spherecast（0x1408ccfc0）— O 全解

```
spherecast(out, world, ray, ?, sphere{radius@+0, ?@+4, type u8@+8, u8@+9}, mask):
  17 槽 mask 遍历; 每槽 cast_shape(sphere{center=全局常量 0x14293e0c0/8 (=0), radius,
             shape tag 静态描述符 @0x142aa3a40}, ray, mask, ...)
  命中 → 竞技场有效性检查（panic @0x142aa3be8/0x142aa3c00）
  法线双向翻转: t = dot(…); 若 < 0 → 各分量异或 0x80000000 符号位（面朝射线侧）
  out = {handle 24B @+0x00, point @+0x18, normal @+0x24, toi @+0x30, marker @+0x34}
    —— 与 raycast 0x40B 布局一致；uv@+0x38 不写（陈旧）
```

### 2.6 RaycastWorld::legacy_spherecast（0x1408ccc90）— O 全解（本轮新解）

```
legacy_spherecast(out, world, ray, ?, sphere{radius@+0, ?@+4, u8@+8, u8@+9}):
  uVar15 = 2 (miss 初值)
  固定 10 槽顺序表（栈上字节数组）: {0, 1, 3, 7, 6, 4, 11, 2, 12, 13}
  for 槽 in 表序:
    cast_shape(sphere{center=全局 (0,0,0), radius=sphere[0] + 1.0@+4 + 0x101 标志}, ray, mask=param_4)
    (底层为 FUN_140b947d0 @0x1408cce97 间接调用)
    if 命中 (状态字节 != 4):
      竞技场有效性检查 {index < world+0x10, arena@+8, entry+0x20 == gen|1} 否则 panic
        (0x142aa3bb8 / 0x142aa3bd0)
      out = {entry[0..1] @+0x00 (16B), entry[2]@+0x10 (24B handle),
             point = origin + toi*dir @+0x18,
             0 @+0x24 (12B: 无法线/toi), marker @+0x30 = 0}
      break
  全未中: out.marker@+0x30 = 2（handle/point 陈旧不写）
```

**布局裁决（CLOSURE 遗留 ±4B 问题收口）**：legacy 变体与标准 0x40B 不是"4B 偏移差"，而是**更小的专用结果**：`{handle 24B, point 16B, 零 12B, marker@+0x30}` —— 省略 normal/toi/uv，marker 在 +0x30（标准布局该处为 toi，marker 在 +0x34）。消费方（critters avoid/move 0x1423179a0/0x142318fa0/0x1423246b0）按此专用布局读。固定槽序 {0,1,3,7,6,4,11,2,12,13} 中槽 2 与 3 出现但位序靠后 —— 与"全量掩码缺槽 2/3"的调用侧观察互补（掩码语义是"查询子集"，此处是"查询序"）。

### 2.7 raycast_w_terrain（0x1408cd430）— O 全解

```
raycast_w_terrain(out, world, ray, terrain_world, ?, mask):
  if mask & 0x20 == 0: return Rapier::raycast(out, world, ray, ..., mask)   # 无地形
  local = Rapier::raycast(&local, ...)                                      # 先碰撞
  if local.marker == 2 (碰撞无命中):
      terrain_hit = terrain_raycast(&local, terrain, ray, ...) (FUN_1409e9a20, terrain 域)
      if !terrain_hit: out.marker@+0x34 = 2
      else: 合成地形命中: out.byte0 = 0xb (Terrain 判别), point, normal=(0,1,0)@+0x24,
            toi@+0x30, marker@+0x34 = 0
  else (碰撞命中):
      terrain_hit = terrain_raycast(cap=local.toi@+0x30, ...)  # 地形以碰撞 toi 为上限
      if !terrain_hit: 整块拷贝 local 0x40B → out (含 marker/toi/uv)
      else: 地形更近 → 合成地形命中 (byte0=0xb, normal (0,1,0), marker=0)
```

### 2.8 upsert_collider_entry（0x140b40b50，<Entity,(Vec3,Quat)> 实例）— O 全解

```
upsert_collider_entry(world, handle(24B), geometry, transform):
  注册表查找 handle → 已有?
  if 已有 && 几何哈希相同: return status=2 (未变)
  if 已有: dispose_collider_mesh_instance(旧) (0x1408d9590)
  新建 parry collider（user_data 编码 handle）→ 槽竞技场插入
  一致性断言（O 串）:
    "collider exists in the physics world but not its collider registry"  (0x142aa3d50)
    "collider was removed from physics world but not its collider registry" (0x142aa4250)
  QueryPipeline::update
```

泛型族 6 实例（v2 名）：`upsert_collider<StairGraphEdge,(Vec3,Quat)>` 0x140252c90、`<WallId,(Vec3,Quat)>` 0x1412453c0、`<DecoratorId,(Vec3,Quat)>` 0x141294980、`<StairGraphEdge,IsoTransform>` 0x141296510、`<DecoratorId,OffsetColliderTransform>` 0x141298110、`<(StairGraphEdge,u32),IsoTransform>` 0x141299ca0；`remove_collider<StairGraphEdge>` 0x140252ad0 ×3（invalidate）。

### 2.9 update_rapier_colliders（0x1408d0490）— I（varnode 卡点，管线闭合）

- 边界 0x1408d0490..0x1408d5e50；入口 = Bevy 系统序言 + puffin scope（0x1408d050c call 0x140c169f6 区）。
- callgraph 管线：parry 几何构建（0x14145a540/0x1416bb980）→ `upsert_collider_entry`（0x140b40b50）→ `dispose_collider_mesh_instance`（0x1408d9590）→ 变更网格查询 ×4（0x140d00770，疑 per-墙/装饰/楼梯/拱）→ hashbrown（0x1428a9ca0）/ hash（0x1405b0d70）。
- 语义：**逐子世界查变更网格 → parry 重建 → upsert/回收**；闭包 0x1408d7ed0（7482B）= arch/floor 网格构造 helper（construct_arch_geo/floor_mesh 族）。

### 2.10 RaycastMesh 族构造 — O（写入面/读取面裁决沿用 CLOSURE）

- `new_convex` 0x1408cec10：parry convex_hull；Some 写 tag=4、0xd0B 堆块@+0x20、vtable@+0x28、hash@+0x70。
- `new_unindexed` 0x1408cf7d0：输入顶点步长 0x24（pos+normal+uv 三 Vec3），生成顺序索引 i,i+1,i+2；写入面：`+0x00/+0x08 parry shape、+0x10/+0x18 indices{ptr,len}、+0x20/+0x28 hash、+0x30/+0x38 法线、+0x40/+0x48 uv、+0x50..+0x68 类型静态、+0x70 hash2`。
- `RaycastMesh{key@+0, Arc@+8, source 24B@+0x10}`；`raycast` 0x1408ceb40：Arc 内 shape vtable `[arc+0x38]` 虚调 `cast_ray_and_get_normal`；无命中 marker=2。

---

## 3. 楼梯（stairs）逐函数算法

### 3.1 StairsState 备份/恢复机制 — O 全解（本轮机器码裁决）

**布局修订（推翻 DIGEST 两处）**：
- 备份区 = **+0xb8 起 0x90B**（= 图的镜像区，与图区 +0x28..+0xb8 等长；DIGEST "+0xb0" 差 8）。
- restore_backup **只写 +0x178**（开头 0、结尾 1），**不写 +0x2f** —— DIGEST "+0x2f 脏字节" 无机器码依据，实为 +0x178 `backed_up` 标志（add_node/remove_node 清零，make_backup/restore_backup 置 1）。
- make_backup 开头有 `if (+0x178 == 0)` 守卫（已备份则幂等跳过）；restore_backup 无守卫。

**make_backup（0x1408fefe0）** = 图 → 备份：
1. `if byte[+0x178] == 0` 才执行；
2. `iter_edges_and_stubs(graph@+0x28, backup@+0xb8, stubs@+0x148)`（实例 0x14157cec0）+ 闭包 `FUN_140b4d570` 收集当前边集合（来自 +0x28）；
3. 读图字段组 {+0x60..0x6c（边计数 4×u32）, +0xa8/+0xb0（hash）}；`FUN_140b4d320` 收集图边表（+0x70 起）；
4. 释放旧备份缓冲（+0x100/+0x108、+0x118/+0x120，Vec 基址重建式 free）；
5. 组装 0x90B blob（当前边 Vec + 图字段）；
6. `memcpy(+0xb8, blob, 0x90)`；
7. `byte[+0x178] = 1`。

**restore_backup（0x1408ff210）** = 备份 → 图：
1. `qword[+0x00] = 0`（变更计数清零）；`byte[+0x178] = 0`；
2. `iter_edges_and_stubs(graph@+0x28, backup@+0xb8, stubs@+0x148)` + 闭包收集备份边集合（来自 +0xb8）；
3. 读备份字段组 {+0xf0..0xfc, +0x100（备份边表 ptr）, +0x138/+0x140}；
4. **释放当前图内部 Vec 缓冲**（+0x70/+0x78、+0x88/+0x90）；
5. 组装 0x90B blob（备份字段 + 备份边表）；
6. `memcpy(+0x28, blob, 0x90)`（整个 GraphMap 区被备份内容替换）；
7. `byte[+0x178] = 1`。

### 3.2 add_node / remove_node — O 全解（64B 手解 + 完整反汇编）

**add_node（0x1408ffd30）**：`qword[+0x00]=0` → `byte[+0x178]=0` → `GraphMap::add_node(self+0x28, n)`（0x140cb10c0）→ 尾调用插桩 `(self+0x148, n, n)`（自环桩进脏边集合，0x1408c863f 族）。

**remove_node（0x1408ffef0，llvm-objdump 全解）**：
1. `qword[+0x00]=0`、`byte[+0x178]=0`；
2. 节点查找：`nodes_len@+0x38==0 → 跳过`；`==1 → 单节点快路径`（+0x30 entry+0x20 == n）；否则 hash 查找（FUN_140106b70 用 map@+0x60/+0x68 + FUN_140b4f360 于 GraphMap@+0x28）；未找到 → 跳过；`idx >= len → panic`；arena@+0x30 空 → 跳过；
3. 读节点邻接：`r14 = arena + 0x10 + idx*0x28`（邻边计数）、`r15 = arena + 0x8 + idx*0x28`（邻接表 ptr，u32 步长 8）；
4. 每邻边：`min/max = cmov 归一化` → 插桩 `(min,max)` 进 **self+0x148**（0x1402c8640 = hashbrown insert）；
5. 插自环桩 `(n,n)` 进 +0x148；
6. `GraphMap::remove_node(self+0x28, n)`（0x14093a220）。

### 3.3 contains_edge_or_stub（0x140900e80）— O 全解

```
contains_edge_or_stub(self, a, b):
  if a == b (节点存在性):
    nodes_len@+0x38: ==1 → 单节点快路径（+0x30 entry+0x20 == a，否则 false）
                     ==0 → false
                     否则: 节点 map（{ptr@+0x60, hash@+0x68}）双次 hash 查找于 GraphMap@+0x28
                           （0x140106b70 + 0x140b4f360）→ 越界 panic（0x142c812b0）
                           → arena@+0x30 空 → true；否则 bVar9 = arena + 0x10 + idx*0x28 == 0
                           （邻接条目空判定）
  else (边/桩存在性):
    (a,b) 归一化 (min,max)
    edges_len@+0x80: ==0 → false; ==1 → 快路径: 单边 entry@+0x78 {min@+8, max@+0xc}
                     否则: FUN_140106840(edge map hash @+0xa8/+0xb0) + FUN_140b4f270(self+0x70)
                           → ==1
```

### 3.4 structural_graph upsert_span（0x140869d20）— O 全解（8 表接线）

```
upsert_span(self, span, collider_handle, height):
  # 旧 span 已有 collider（span→collider map @+0xc0 命中且判别 != 0x11）:
  for 旧 collider 的每邻边（iter @0x14093b1c0，判别 0x11 终止）:
      插入 connections 集 @+0x150（FUN_141265a10）
  从 collider→span map @+0x90 移除旧 span（FUN_140b77fd0）
  graph.remove_node(prev) — 断言，失败 panic "assertion failed: self.graph.remove_node(prev)"
    （0x142a96b48, 0x2e=46 字符）
  从 @+0xf0 表移除旧条目（FUN_140b787d0）
  # 新 collider:
  graph 节点确保（FUN_14093afc0）
  collider→span map @+0x90 插入（FUN_14152c190）
  span→collider map @+0xc0 插入（FUN_14152ba60）
  span→height @+0xf0 插入（FUN_1412668d0, height=param_4）
  connections @+0x150 插入 collider（FUN_141265a10）
```
表分布（本轮复核）：+0x90 collider→span、+0xb0 collider map hash、+0xc0 span→collider（= `span_to_collider_handle`）、+0xe0 span hash、+0xf0 span→height、+0x110 hash、+0x150 connections；断言串 `"assertion failed: self.span_to_collider_handle.remove(&span).is_some()"`（0x142a96c50）与 `"assertion failed: self.graph.remove_node(collider)"`（0x142a96c00）同源。

### 3.5 stairs_assembly_params / invalidate — 算法要点

- `AlignKind::position_offset`（0x140959f00）：判别 byte0；变体 1 = {flag u8@+1, Vec2@+4, Vec2@+0xc} 组合 + 常量缩放（重建 stub 返 ZERO 错）。
- `PlayermadeStairsAssemblyParams::length_ws`（0x140959ca0）：点/高 zip → 链式 Vec3 段长求和。
- `invalidate_dirty_stair_artifacts`（0x140b37180，3455B）：脏边 drain（+0x148 集合）→ 每边 `remove_collider<StairGraphEdge>`（0x140252ad0）×3 + Arc drop + RtMesh 删 + `contains_edge_or_stub` 决定 + `StairsVisualState::remove_edge`（0x140aa8620）—— wp21 合同一致。
- **stairs_assemble 接线**（wall 域实例化，本域不重挖）：`iter_shape_overlaps` 泛型在 stairs_assemble 的 4 个实例（calculate_distortion_from_structural_graph 闭包 0x14129b780/0x14129bad0；playermade_stairs_assemble closure_env$21 0x140254890/0x140254aa0）= QBVH 相交遍历：候选 collider 与目标形状逐对 `should_intersect_collider_meshes` 过滤（RaycastGeoSource 判别）→ parry 相交 → 回调闭包（畸变计算/装配决策）。`SubworldIndex::to_bitflag = 1 << 槽号`（Terrain=槽5→0x20）在该族内联使用。

---

## 4. ivy（systems/ivy + resources/ivy）

### 4.1 ivy_grower 10 相跳表状态机 — O（本轮字节级验证）

**跳表 @0x142ab3be0（10 项 int32 相对偏移，基址=表自身）**，本轮回读确认目标：
| 相 | 目标 | 语义（反汇编） |
|---|---|---|
| 0 | 0x14094C09F | 守卫相：`xor 状态字节 bit0`（+0x48）、经 `[state+0x10]` 写回相值（+0x1c）→ bit0 置位走守卫体 0x94ac51，否则主体 0x94c0c0 |
| 1/3/4 | 0x14094AC20 | 主入口（共用）：bit0 置位 → 0x94ac51（守卫），否则 → 0x94c0c0（主体） |
| 2 | 0x14094AC31 | 计时入口：`ucomiss [state+0x11c], 0` → ≥0 走主体 0x94c0a6，否则守卫检查 |
| 5 | 0x14094F2F2 | 二级跳表（0x142ab3bf4，r8 索引）分派 |
| 6-9 | 0x14094F306/31C/… | 尾部入口：串+长度 → `jmp 0x140d86b90`（panic 类，串 @0x142ab3ad0/0x142ab3b50 等） |

分派：`phase = *(u32*)(world+0x118)`（Bevy world 内状态槽）→ `jmp [表 + phase*4]`（0x14094aa1b/0x14094aa2f 原指令）。主闭包 0x14094AC20 生长管线（CLOSURE e8 扫描）：每株 ivy → `last_point`（0x1408694f0）→ `IvyDirectionProposer::get_direction`（0x141234590）→ `closest_segment_index`（0x14094c800）选墙段 → 墙跳检查 → `add_point`（0x140869460）推进。状态结构（相体反汇编）：{+0x10 ptr（相值写回通道）, +0x1c 相值, +0x48 守卫标志 bit0, +0x11c 计时 f32}。

### 4.2 closest_segment_index（0x14094c800，666B）— O 全解

```
closest_segment_index(out, point.xy, segments(56B 步长), count, max_height):
  # 段布局（7 qword = 56B）：{p0.xy@+0, p1.xy@+8, f32@+0x10, f32@+0x14, h0@+0x18, h1@+0x1c, 2 qword 未知}
  #    @+0x10/+0x14 两 f32 打包进结果（语义 U，疑段标识/uv）
  过滤1（机器码原文）：h0 ≤ max 或 h1 ≤ max → 跳过（保留两端点均 > max 的段）
  过滤2：t 处插值高度 h1·t + h0·(1−t) > max → 跳过
  len² = dx²+dy²；t = dot(P−P0, D)/len² 夹取 [0, 1.0]（_UNK_142925904 = 1.0，夹取下限 0）
  断言：夹取后 t ∈ [0,1]（机器码恒真，panic 串 @0x142aa53ed/0x142aa5468，0x2a=42 字符）
  距离度量 = |(P0−P)·单位方向|（沿段方向投影距，另一项经 0 常量折叠）
  保留最近段: out = {found u64@+0, index u64@+8, dist f32@+0x10, h 打包 u32@+0x14}
```
注意：过滤 1 与过滤 2 在 t∈[0,1] 下看似互斥（两端点 > max ⇒ 凸组合 > max）—— 原语义疑为高度坐标取负向/阈值方向相反（U 待调用方数值确认）；机器码行为以此为准。

### 4.3 ivy 其余系统 — 算法要点

- `ivy_spawner`（0x14094cf90，38KB）：try_random_ivy_root / try_ivy_root_next_to_cursor / is_point_inside_wall_segment；调 `closest_segment_index`（0x14094c800）+ `func_0x000140c9cdc0` ×6（几何采样，疑墙段取点）。
- `ivy_pruner`（0x140997410，70KB）：墙空间哈希校验（"Spatial Hash returned a wall id " 0x142abb028）+ 逐点 `is_point_still_valid`（0x140998dc0，内含 closest_segment_index 复检）+ `ivy_pruner_cmd`（0x140999330，RemoveIvyCmd 消费）+ `remove_ivy_at_location`（0x140999860）。
- `ivy_leaf_spawner`（0x140996080）：消费 `IvySegmentDoneGrowingMsg`（panic "Received IvySegmentDoneGrowingMsg with ivy id " 0x142abaeb0）→ 沿末段放叶（`IvyLeafStorage::add` 0x1408fe280，104B 叶）。
- `IvyDirectionProposer::get_direction`（0x141234590）：3×noise3d + 3×uniform 混合 → θ∈[−40°,+40°] → (cosθ, sinθ)。
- `WallCoord::into_world_space`（0x140b199e0）：PublicWallState 曲线采样 + 法向偏移 0.63×0.5（按侧 ±）。

---

## 5. paths 状态机

### 5.1 DrawState::update（0x140ae4bd0）— O 全解（llvm-objdump）

```
update(self(mode u8 @+0), ui_mode):
  r = clamp(ui_mode − 2, 0, 3)              # addb −2 + cmovb 3
  if r == 0: mode = 0                       # testl/je
  elif r ∈ {1,2}: mode = (mode−1 ≥ 2) ? r : 2    # cmovael（章内 "min(mode−1, raw)" 的精确机器码形态）
  else (r == 3): mode = (mode−1 < 2) ? 3 : 0     # setb + lea ×3
  mode 写回 self+0；返回
```

### 5.2 draw_path（0x140ae4d00，15KB）— O 主链

```
draw_path(mode 资源, ui_mode_str, raster 资源, cursor, history, …):
  模式字节 → lVar14 = mode−5 ∈ {2,3,4} 才处理（mode ∈ {7,8,9}，否则 return）
  lVar14 == 4（=9）: 标志检查 (flags & 5) != 0 → return
                       brush = *param_13 × 0.5525（_UNK_142a95dd0）
                       assert brush.is_finite()（"assertion failed: brush_size.is_finite()" 0x142ae2e06,
                         0x28=40 字符；第二断言 0x142ae2f1b）
                       UI 串匹配 "eraser"（0x142ae2e80, len 6, FUN_14092df50）
  lVar14 == 3（=8）: brush = *param_8（另一资源面）; 匹配 "cobblestone"（0x142ae2f10, len 0xb）
  lVar14 == 2（=7）: brush = *param_8; 匹配 "path"（0x142ae2ef0, len 4）
  匹配记录 = FUN_14092df50 返回 {+0x18 标志字节(>1 = 有附带语义), +0x38 名称 ptr, +0x19 变体字节}
  笔刷矩形: rect = {cursor 位点（读 param_9 {flag bit0, x@+4, z@+0xc}）,
                     尺寸 = brush × 0.8（_UNK_142a16cb8）}（func_0x000140c97540）
  绘制状态推进（FUN_140ad2970 谓词）→ 历史命令 Vec push（cap-first, 8B 元素）:
      {命令 id, 计数器 +1}；条件二推（记录标志 >1 且名称匹配且变体 != 2）
  变体互擦（erase_other_path_variants）: cVar15 = 记录变体字节 → clamp(variant−2, 0..3)
      → 若当前模式字节 ∈{1,2} → 置 2 + 按记录路径擦除
  9 个 world_raster::draw_raster 单态分派（0x140ab35f0/0x140ab6de0/0x140ab8090/0x140abba80/
      0x140ab2340/0x140ab0f10/0x140ab9340/0x140ab4750/0x140aba770 — 每变体绘制+擦除组合）
```

---

## 6. 常量表（本轮 rne-read 证实）

| 常量 | 值 | 用途 |
|---|---|---|
| `0x14362c3a8` | 全局帧计数器 | flush_colliders/update_rt_world/upload/build 的日志门限（>4） |
| `_UNK_142925904` | 1.0f | closest_segment_index t 夹取上限；legacy_spherecast 球参数@+4 |
| `_UNK_14293e0c0/8` | 0,0 | spherecast/legacy 球心全局常量 |
| `_UNK_1429258f0/f4` | 0x80000000 ×2 | 法线符号翻转掩码（spherecast/legacy 命中法线） |
| `_UNK_142a95dd0` | 0.5525f | draw_path 笔刷缩放 |
| `_UNK_142a16cb8` | 0.8f | draw_path 笔刷半径 |
| 跳表 0x142ab3be0 | 10×int32 | ivy_grower 相分派（§4.1） |

---

## 7. 与既有证据的差异（本轮新增/裁决）

1. **restore_backup +0x2f 误记**：机器码（llvm-objdump 0x1408ff36e）只写 +0x178（开头 0/结尾 1）；DIGEST §3.1 "+0x2f 脏字节" 应更正为 **+0x178 backed_up 标志**（add_node/remove_node 清零；make_backup 守卫 + 置 1；restore_backup 置 1）。备份区起点 = **+0xb8**（非 +0xb0）。
2. **legacy_spherecast 结果布局**：CLOSURE/DIGEST 的"±4B 偏移差"裁决为**专用小结果** `{handle 24B, point, 零 12B, marker@+0x30}`；固定槽序与球参数 {radius, 1.0, 0x101} 机器码证实。
3. **closest_segment_index 步长 56B、h0@+0x18/h1@+0x1c**（review-audit 已纠）；两过滤互斥疑点待调用方数值裁决。
4. **ivy_grower 跳表字节级验证**：CLOSURE 目标表正确（本轮回读 10 项 + 相体反汇编补语义：相 0 守卫翻转、相 2 计时、相 5 二级跳表、相 6-9 尾部入口）。
5. **marker 三值语义**（0=无 uv 命中 / 1=uv 已插值 / 2=无命中）—— DIGEST "0/2 两值" 精化。
6. **raycast_simple 小结果 0x28B**（toi@+0x24）—— 与 0x40B 结果族为不同结构（重建需两结果类型）。
