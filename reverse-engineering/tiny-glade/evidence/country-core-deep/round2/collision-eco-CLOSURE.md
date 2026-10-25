# collision-eco 域 round2 闭合报告（6 个 U 项）

> 方法：v2 权威符号表 + 原字节手解 + rne-read + callgraph + 定点 MCP（tg_a7c）。
> 状态：**CLOSED**=证据闭合 / **PARTIAL**=部分闭合+残余 U / **OPEN**=卡点。

## 1. SubworldIndex 17 槽完整名序 — PARTIAL

**已闭合（O）**：
- 判别→槽映射表两处同值：0x142aef03c（upsert_collider_entry 读）与 0x142aa4418
  （RaycastGeoSource::subworld 读）：disc 0..16 → slot {0,9,1,2,8,14,3,7,6,10,4,5,11,12,13,15,16}。
- Terrain = 槽 5 三证锚定：mask 位 0x20 触发地形射线（raycast_w_terrain）+ 地形命中写
  handle byte0=0xb（disc 11）+ table[11]=5。
- SubworldColliderHandle 变体名（Debug 元数据 @0x142ab71e0 区，O 串）：Roof{flag,gable}、
  DecoratorCollision{decorator_id,addr}、DecoratorInteraction、StairInteraction、Column、Stairs、
  Fence、Arches、Tree、Terrain、AutoClutter{entity}、StairSegment{graph_edge}、StairSupport、
  SupportedStairStructural{point_idx}、UnsupportedStairStructural（+Floor 待补）。
- eq（0x1408cea30，v2 无符号=内部函数）17 负载变体形状分组：{0,1,6,7,8,9,12}=qword@+8、
  {2}=qword@+8+2B、{3,4,5}=u32@+4+qword@+8+opt-qword@+0x10、{10,11}=2B、{13,14}=qword@+4、
  {15,16}=qword@+8。
- `SubworldColliderHandle::new`（0x1408cc620）= 写 {u32@+0=index, u32@+4=generation, u8@+8=slot}
  —— get_collider（0x1408cdb50）按 {index,gen,slot byte@+8} 回查 rapier 竞技场 ✓ 自洽。
- 观察到的调用侧掩码：1（单槽 0）、0x77fb（=槽 {0,1,3..12}，缺 2,13,14,15,16）、
  0x3f43（=槽 {0,1,4..13}，缺 2,3,14,15,16）——两个"全量"掩码都排除槽 2、3。

**残余 U（卡点注明）**：
- 槽 2、3 的名字无法从本域证据裁决（"全量"掩码排除它们，语义需调用者侧：候选
  StairInteraction/StairSegment 或两 StairStructural 槽，I 不臆造）。
- 15 名串 blob @0x142aa4360 + 字节/跳表 @0x142aa4420/0x142aa4428 + fmt 分派函数
  @0x1408d99e0（RIP 扫描定位）：若为 SubworldIndex 的 fmt，其声明序（Terrain 第 4）与
  槽 5 冲突——**裁决：该 blob 属另一 15 变体枚举（"Floors"/"StairSegments" 复数名
  提示渲染/查询组），非 SubworldIndex**；SubworldIndex 的 fmt 未在符号表/引用中定位。
- 建议：由消费掩码的域（terrain-water/wall/ui-input 的调用者反编译）逐一锚定槽名。

## 2. update_rapier_colliders（0x1408d0490）varnode 失败 — PARTIAL

- 重试 2 次（tg_a7/tg_a7c）均 `Unable to find unique hash for varnode`——Ghidra 数据库
  确定性失败，非瞬时。
- **函数边界（O）**：callgraph 键集显示 0x1408d0490..0x1408d5e50（≈0x19c0=6.6KB，下一
  函数起点 0x1408d5e50），其后 28 个内部小函数（0x1408d5e50..0x1408d7ed0，各 0xE0-0x2C0B）
  + 闭包 0x1408d7ed0（arch/floor 网格构造，已落盘）。
- 入口字节（O 手解）：标准 Bevy 系统序言（保存 8+ XMM、sub rsp 0x4a8、4 参数拷贝）→
  首个调用 = puffin scope 检查（0x1408d050c call → 0x140c169f6/0x140c16a00 区）。
- 调用管线（O：callgraph callees）：parry 几何构建（0x14145a540/0x1416bb980）、
  `upsert_collider_entry`（0x140b40b50）、`dispose_collider_mesh_instance`（0x1408d9590）、
  变更网格查询（0x140d00770 ×4，疑 per-墙/装饰/楼梯/拱查询）、hashbrown（0x1428a9ca0）、
  hash（0x1405b0d70）、alloc/memcpy、puffin。语义：**逐子世界查询变更网格 → parry 重建 →
  upsert/回收**。
- 注册/调度点（O：callgraph）：0x140b3cb20、0x1414f15b0。
- 残余 U：系统参数面（哪些 Query/ResMut）与逐分支细节——需 asm 级恢复或换反编译器
  （radare2 pdg / reva），本域以管线证据闭合语义面。

## 3. add_node（0x1408ffd30）varnode 失败 — CLOSED（64B 全手解）

```
mov esi,edx; mov rdi,rcx
mov qword[rcx],0            ; +0x00 清零（同 remove_node）
mov byte[rcx+0x178],0       ; backed_up 清零
add rcx,0x28; call 0x140cb10c0   ; GraphMap::add_node(self+0x28, node)（petgraph 内部）
lea rdi,[rdi+0x148]         ; rdi = self+0x148
tail-call 0x1408c863f 类插桩(rsi=node, r8=node)   ; 自环桩 (node,node) 进脏边集合
```
- 与 wp21 合同 add_node 0x8ffd30 ✓ 完全一致；DIGEST §3.1 已更新（含 +0x29→+0x148 更正）。
- 顺带更正：remove_node 的 `param_1+0x29`（undefined8*）= 字节 **+0x148**，非 +0x29。

## 4. RaycastMeshData 写入面 vs 读取面 ±8 偏移 — CLOSED（裁决：两个不同结构）

- 写入面（new_unindexed 0x1408cf7d0，param_1）：+0x00/+0x08=parry shape、+0x10=indices
  ptr、+0x18=indices len、+0x20=hash、+0x30/+0x38=法线、+0x40/+0x48=uv、+0x70=hash2。
- 读取面（rapier_hit_to_raycast_world_result 0x1408ce4a0，lVar7）：+0x18=indices len(0 检查)、
  +0x20=indices ptr、+0x40=法线 ptr(0=无)、+0x48=法线 len、+0x50=uv ptr(0=无)、+0x58=uv len。
- 两侧 indices 偏移差 +0x10 且法线/uv 差 +0x8——**不存在单一平移对齐**，故为两个独立类型：
  lVar7 = 命中视图（0x60B，三对 {ptr,len}，由 Rapier::raycast 从 collider user_data→
  注册表→网格解出），非 RaycastMeshData。重建代码单一 RaycastMeshData 模型需新增命中
  视图类型。佐证：RaycastMesh::raycast（0x1408ceb40）显示 RaycastMesh{key@+0,Arc@+8,
  source24B@+0x10}，parry shape 经 Arc 内 vtable 虚调 cast_ray_and_get_normal。

## 5. ivy grower 主闭包（跳表 0x142ab3be0 分派目标）— CLOSED（定位+边界）

- **PDB 边界（O）**：`ivy_grower` = 0x14094a910..0x14094c7fb（**7915B**，evidence/pdb-structure/
  procedures.json code_size）；`closest_segment_index` = 0x14094c800..0x14094ca9a（666B）。
  即 10 个相位入口全部位于**单一函数体内**（Ghidra 无独立函数 → NotFound 合理）。
- 跳表 @0x142ab3be0 = **10 项**（第 10 项后即 rustc 路径串）：{0x14094c09f, 0x14094ac20,
  0x14094ac31, 0x14094ac20×2, 0x14094f2f2, 0x14094f36c, 0x14094f31e, 0x14094f356, 0x14094f308}。
  分派索引 = `*(u32*)(world+0x118)` —— ivy_grower 是 10 相状态机（每相一入口）。
- **主闭包 = 0x14094ac20**（相 1/3/4 共用入口）：e8 扫描证实其在 0x14094b06a 调
  closest_segment_index（0x14094c800）、0x14094ad17 调 0x141309480；callgraph（0x14094a910
  函数体）另含 last_segment_point_ids（0x140869540）、IvyDirectionProposer::get_direction
  （0x141234590）、last_point（0x1408694f0）、WallCoord::into_world_space（0x140b199e0）、
  Vec grow（0x140562c30）、hashbrown（0x141469a20/0x141467ce0）。0x14094c09f（相 0）=
  puffin/守卫相（puffin 调用 + panic 路径）。
- 生长管线（O 拼接）：每株 ivy → last_point → get_direction 提议 → closest_segment_index
  选墙段（28B 步长段，高度 ≤ max_ivy_grow_height 过滤）→ 墙跳检查 → add_point 推进。
- 残余 U：各相具体转移条件与数值（步长/随机阈值）——需 asm 级恢复；语义面已闭合。

## 6. 附带闭合/更正（本轮新发现）

- `Rapier::get_collider`（0x1408cdb50）与 `get_collider_mesh`（0x1408cdbd0）均为 rapier
  竞技场回查（entry 0x28B，gen 标签 @+0x20）；前者返回游戏侧 0x28B 视图
  {mesh×3 qword@+0x34/+0x3c/+0x44, f32@+0x4c, gen|1@+0x20, index@+0x24}。
- `RaycastMesh::raycast`（0x1408ceb40）落盘（round2 decomp）。
- `xz_flatten_aabb`（0x1408d98a0）手解：输出 {min.x, min.y, max.y}（AABB xz 展平）。
- stairs 域修正：stubs 集合 +0x148（非 +0x29）；backed_up +0x178；add_node 全解。
- legacy_raycast/legacy_spherecast 掩码=0/固定 10 槽序（0x1408cc680 0x1408cc770 已落盘）。

## 卡点汇总（OPEN）

| 项 | 卡点 | 建议 |
|---|---|---|
| 槽名序（槽 2/3 等 12 个） | 本域无锚点证据 | 由调用掩码的域（terrain-water/wall/ui-input）反编译交叉锚定 |
| update_rapier_colliders 体 | Ghidra varnode 确定性失败 | radare2 pdg / reva 或 asm 分块恢复 |
| grower 各相转移数值 | 主闭包无函数边界 | 同上（asm 级） |
| 15 名 blob 归属枚举 | 与 SubworldIndex 槽序矛盾 | 找其 fmt 的调用点/实现（0x1408d99e0 邻域） |
