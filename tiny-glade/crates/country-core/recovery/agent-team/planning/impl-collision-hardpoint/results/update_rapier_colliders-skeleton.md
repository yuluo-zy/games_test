# update_rapier_colliders (0x1408d0490) — 行为级骨架证据

日期: 2026-10-17
角色: impl-collision-hardpoint
证据: 
- `hardpoint-update_rapier_colliders.disasm.txt`（ghidra shared `disassemble_function`，4630 条指令，284KB）
- `b1-rt-findmesh.result.json`（41 callee 清单，fresh）
- `h1-reva-hardpoint.result.json` / `h2-reva-pcode.result.json`（ReVa `--write` worker
  `impl-collision-hardpoint`：签名修复 `void update_rapier_colliders(longlong * this)` +
  fresh decompile / High P-code 仍 `Unable to find unique hash for varnode`）

## 状态: 未收敛（decompile-hardpoint），行为级骨架 + 逐段 disasm 落盘，登记不伪造

`fresh decompile`（ghidra shared + ReVa 写 worker）均返回 `Recoverable Error: Unable to find
unique hash for varnode`。ReVa 已修复签名并 save（h1 step0 success），High P-code 尝试
（h2）在 HighFunction 构建阶段同样失败（同一错误）。分段 disasm 已落盘
（`hardpoint-update_rapier_colliders.disasm.txt`），call 序列 + 关键分支已分析如下。
**未实现**——按 2026-10-16-logic-verify-protocol 标 `未实现-需重做`（行为骨架，不作判等）。

## 函数面（ABI，从 prologue 寄存器使用锚定）

```
RCX/RDX/R8/R9 + RSP+0x28/+0x30（栈参数）：4 寄存器 + 2 栈参（bevy Query + 系统参数面）
prologue: SUB RSP,0x4a8; 保存 XMM6..XMM15 至 [RBP+0x380..0x410]
```

## 行为骨架（call 序列驱动，逐段对应 disasm）

| 段 | 地址区间 | 调用 | 行为 |
|---|---|---|---|
| 0 | 0x1408d0490-0x1408d0736 | `are_scopes_on`/`rfind`/`slice_error_fail`/`VAL`/`begin_scope` | puffin scope "update_rapier_colliders" |
| 1 | 0x1408d0962-0x1408d0f3f | `get_changed_ticks_slice_for`×2 → `calculate_hash<[u32;3],UVec3>`/`cuboid`/`from_translation`/`hash_one<GlobalId>`/`upsert_collider_entry<Entity,(Vec3,Quat)>` | 段 1 = bevy `Changed<…>` 查询循环：每个变更实体按矩形尺寸构造 cuboid `RaycastMeshData`（`[RCX+0x50]/[RCX+0x54]/[RCX+0x5c]-[RCX+0x58]` 半边长、`×0.5` 缩放）→ `HashMap<Entity,…>` 键查（hashbrown SIMD 探测，world+0x3e88/+0x3e90）→ `upsert_collider_entry` |
| 2 | 0x1408d11e5-0x1408d12b9 | `dispose_collider_mesh_instance`/`drop_slow<RaycastMeshData>`/`reserve_rehash`/`hash_one` | 段 2 = 移除实体的 collider 释放 + 槽脏 + 发布 |
| 3 | 0x1408d1605-0x1408d1fd6 | `get_changed_ticks_slice_for`×2 + `hash_one`/`reserve_rehash`/`dispose`×5 | 段 3 = 第二组件族变更循环（多处 dispose + reserve_rehash，monomorphization 重复体） |
| 4 | 0x1408d2239-0x1408d269a | `construct_arch_geo`×N（0x68 B/arch 循环）→ `new_unindexed` | 段 4 = arch 几何：逐 arch `construct_arch_geo` 收集三角 → `new_unindexed` 建网格 |
| 5 | 0x1408d279a-0x1408d3a81 | `reserve_rehash`/`dispose`/`drop_slow`/`cuboid`?/`new_indexed`/`hash_one`/`__rust_alloc`/`memcpy` | 段 5 = 第三组件族（new_indexed 网格）变更处理 |
| 6 | 0x1408d35bc-0x1408d36e6 | `xvy`/`as_points3`/`from_iter<Vec3,circle>`/`as_points2`/`from_iter<Vec2,circle>`/`grow_one<Vec3>`/`grow_one<(u8,u32)>`/`from_iter<[u32;3],floor_mesh>` | 段 6 = 圆/地板网格：`circle::as_points2/3` 生成 2D/3D 点位 → `floor_mesh::closure_env_0`（TupleWindows 索引装配）→ 后续 `new_indexed` |
| 7 | 0x1408d398c-0x1408d3e90 | `new_indexed`/`upsert_collider_entry`×4/`reserve_rehash`×4 | 段 7 = 圆/地板网格 collider upsert（×4 入口，多组件） |
| 8 | 0x1408d46e1-0x1408d4a10 | `get_added_ticks_slice_for`×2 + `dispose`/`drop_slow`/`reserve_rehash` | 段 8 = bevy `Added<…>` 查询循环：新增实体处理 |
| 9 | 0x1408d4e9f-0x1408d5262 | `get_changed_ticks_slice_for`×4 + `from_iter<Triangle,roof_trigger_volume::triangles>` | 段 9 = 屋顶触发体积：`roof_trigger_volume::triangles` → `from_iter<Triangle>`（0x24 B/元素分配） |
| 10 | 0x1408d5386-0x1408d5615 | `dispose`/`memcpy`/`new_unindexed`/`hash_one` | 段 10 = 屋顶触发体积网格 `new_unindexed` |
| 11 | 0x1408d57cc-0x1408d5aaf | `upsert_collider_entry`/`reserve_rehash`/`dispose`/`drop_slow` | 段 11 = 屋顶触发体积 upsert + 最终 dispose |
| 12 | 0x1408d5b3a-0x1408d5e3c | `hash_one`/`VAL`/`end_scope`/panic 系列 | puffin scope 结束 + 返回 |

## 关键数据流（逐段证据）

- **段 1 cuboid 构造**（disasm [386..432]）：半边长 `XMM7=[RCX+0x50]`, `XMM6=[RCX+0x54]`,
  `XMM0=[RCX+0x5c]-[RCX+0x58]` → `×XMM8/XMM9` 缩放 → `calculate_hash<[u32;3],UVec3>`
  → `cuboid(XMM7,XMM0,XMM6)（×0.5 于 new_cuboid 内部）` → `from_translation` →
  RaycastMeshData 形状变体栈装配 `{[+0x00]=1, [+0x08]=1, [+0x10]=4, [+0x18]=0, [+0x20]=4,
  [+0x28]=0, shape@+0x30, …}`（与 world.rs `new_cuboid` 的 tag/kind=4 布局不同——这是
  `upsert_collider` 的内联构造面，登记分歧）→ 0x90 B 分配 + memcpy → `hash_one<GlobalId>`
  → world collider map（+0x3e88/+0x3e90）键查。
- **world 参数**：`[RBP+0x370]` = `&RaycastWorld`（+0x3e88/+0x3e90 = collider map data/mask，
  +0x3e98 = table len），Rust `RaycastWorld` 当前无此 map 字段（`HashMap<Entity,
  ColliderMeshInstance>` 由 upsert/remove 调用方持有）——**登记**。
- **段 4 arch**：`Vec<ArchGeo>`（0x68 B/条）循环，`construct_arch_geo(arch, &mut triangles,
  &mut meta)` 逐条 → `new_unindexed(triangles, uvs)`。
- **段 6 圆/地板**：`as_points2(ptcount)/as_points3` → `Vec<Vec2>/Vec<Vec3>`；
  `floor_mesh::closure_env_0` = `TupleWindows<Range<usize>>` 索引；`grow_one<Vec3>`/
  `grow_one<(u8,u32)>` 为 push 扩容。
- **段 9 屋顶触发体积**：`from_iter<Triangle, roof_trigger_volume::triangles>` 收集三角
  （`[RSI+0x8]` + `[RSI+0x10]`×0x24），`ROOF_TRIGGER` 半边长来自 `[RBP+0x268]+0x4c`。

## 结论

- **收敛性**：decompile-hardpoint（varnode hash 错误）在 ghidra shared + ReVa 写 worker
  （签名修复后）均不可逆。High P-code 不可得。4630 指令 + 多个内联 monomorphization
  （bevy 查询 + 5 组件族 collider 构造），语句级判等不可行于本卡。
- **交付**：行为骨架 + 全量 disasm 落盘；`update_rapier_colliders` 登记
  `未实现-需重做`（行为级骨架，非伪造）。
- **后续**：candidate 路径——`half_timbering` 式更完整 typed prototype（给段 1/4/6/9 的
  内联结构打类型）后逐段判等；或拆成 5 个组件族循环函数再反编译（Ghidra 无此拆分能力）。
