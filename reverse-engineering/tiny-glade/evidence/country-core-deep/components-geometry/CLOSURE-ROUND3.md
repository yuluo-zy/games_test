# components + geometry 未探究项闭合（round3，2026-10-15）

> 闭合 agent：components 未探究闭合（review 组 E）。
> 范围：`chapters/components/**`（mod.rs / drawable.rs / transient_mesh.rs 3 章）+ `components-geometry/ALGORITHMS.md`。
> 结论分级沿用 O/I/U。**不改 reconstruction 源码**（差异仅登记，由集成 owner 裁决）。
> 本文件登记 round3 闭合结论；`ALGORITHMS.md` 与章节已就地更新。

---

## 1. transient_meshes 布局裁决（groupE #5）—— 已闭合（O）

**裁决：mesh_library 章与 components 章描述的是同一队列，两侧各对一半、各错一半。**

- `AssetMeshLibrary.transient_meshes`（字段 @+0x18，访问器 0x140002450 = `lea rax,[rcx+0x18]; ret`）=
  `Arc<TransientMeshRegistry>`，其中 `TransientMeshRegistry` = **`std::sync::Mutex<Vec<u64>>`**（**32B = 0x20**）。
- 内层（注册表基址）：Mutex state 字节@+0、poison@+1、pad、Vec{cap@+8, ptr@+0x10, len@+0x18}（cap-first 游戏 Vec，元素 = u64 mesh handle id）。
- ArcInner 0x30：strong@0/weak@8/registry data@+0x10（state@+0x10、poison@+0x11、cap@+0x18、ptr@+0x20、len@+0x28）。
- `TransientMesh` = `{ registry: Arc<TransientMeshRegistry>@+0, mesh_handle: u64@+8 }`（16B）。

**两章对/错判定：**

| 侧 | 对 | 错 |
|---|---|---|
| mesh_library 章 | Arc 包装、ArcInner 0x30 偏移、std futex Mutex 标签、字段 @+0x18 | 元素类型 U（占位 `Handle<Mesh>`）→ 实为 **u64 mesh handle id** |
| components 章 | 注册表内层偏移（state@0/poison@1/Vec@+8）、元素 u64、cap-first、len 清 0 保留容量 | 总尺寸"24B" → **32B**；"parking_lot 锁" → **std::sync::Mutex**；把 `TransientMeshRegistry` 当独立 `Res` 资源 |

**证据链（O）：**
1. drain 0x1412e9e70（expansion-b1/decomp + B-mesh-helpers/1412e9e70.asm/.c，RNE 基线）：`CMPXCHG.LOCK byte [RBX],1` 锁 state@0、poison@+1、len@+0x18/ptr@+0x10/cap@+8、循环 8B 元素直调 `AssetMeshLibrary::remove`(0x14092d750)+`AssetVAOLibrary::remove`(0x1409247a0)、尾部 `MOV qword [RSI+0x18],0` 仅清 len；慢路径 0x1428d8750、唤醒 0x1428d8810。
2. PDB public-symbols：0x1428d8750 = `std::sys::sync::mutex::futex::Mutex::lock_contended`、0x1428d8810 = `std::sys::sync::mutex::futex::Mutex::wake` → **锁 = std::sync::Mutex（futex 实现），非 parking_lot**。
3. drop 0x140899f90 decomp：`[rcx]`=ArcInner（TransientMesh.registry@+0）、`[rcx+8]`=mesh_handle、Mutex state@ArcInner+0x10、Vec cap/ptr/len @+0x18/+0x20/+0x28。
4. PDB FunctionSystem 符号（0x1405e5e70 drop_in_place 族）揭示 `delete_dropped_transient_meshes` 系统参数面 = **仅 `(ResMut<AssetMeshLibrary>, ResMut<AssetVAOLibrary>)`**——无 Res<TransientMeshRegistry>，注册表不是独立资源。
5. 调用方 `construct_wooden_gate_doors` 0x141210110（141212940 `AssetMeshLibrary::add` → 141212981 `transient_meshes()` → 141212993 `new_with_registry(handle, &Arc)`）：注册表 = AssetMeshLibrary 的 Arc 字段。

**连带修正：**
- `new_with_registry`（0x140aa8200）实签名 = **`new_with_registry(mesh_handle: u64, registry: &Arc<TransientMeshRegistry>) -> TransientMesh`**（16B 完整函数：`mov rax,[rdx]; lock inc [rax]` Arc clone 溢出→ud2；`mov rdx,rcx; ret`，RAX:RDX 返回）。0x140aa8210 起的 0xd8 栈帧是相邻独立函数，不是本函数主体。reconstruction `(registry: Arc, mesh_handle)` 参数序/按值不符——登记。
- reconstruction 差异：`TransientMeshRegistry` 作 `Res` 资源（实为 AssetMeshLibrary Arc 字段）；drain `std::mem::take`（实为仅清 len 保容量）；毒化 `if let Ok`（实为 `.lock().unwrap()` panic）。登记，不改源码。

---

## 2. mod.rs 组件计数「45 mod.rs」off-by-one（groupE #16）—— 已闭合（O）

- regmap 扫描（comp_register_map.py）：`country_core::components::`（mod.rs，不含 drawable/transient_mesh）register_required_components 恰 **44 个**（0x140844f20 … 0x1408579f0）。
- 合计 **44 mod.rs + 12 drawable + 1 transient = 57**；业务函数 54 = 44 register + 9 clone_behavior + 1 serialize（与 mod.rs 章"函数总计 76：业务 54"一致）。
- 头"45 个 ECS 组件类型 / 45 mod.rs"为 off-by-one。已改：`chapters/components/mod.rs.md` 头 → **44**；`ALGORITHMS.md` §1 → **44+12+1=57**。

---

## 3. clone_behavior 归类收尾（groupE §2.5）—— 已闭合（O）

- regmap 213 行中 `clone_behavior` 恰 **17 个 per-impl 符号**，与 ALGORITHMS §2 的 17 个 Custom 一一对应（各带地址，见 ALGORITHMS 更新）：
  DrawTopology(0x14082cac0)、NaniSolidTexture(0x140831030)、PrevTransform(0x140831730)、ClothlineCloth(0x14084cc00)、GothicWindowBrickDeformation(0x14084d300)、BrickColor(0x14084da00)、HistoryButton(0x140851f70)、Smoke(0x140854920)、RoofFlagCloth(0x140855020)、WallFlagCloth(0x140855720)、LanternLightParams(0x140856c00)、BrushPreview(0x140867080)、Roof(0x1408e6af0)、CurvedHalfTimberInstanceData(0x140990af0)、ColorId(0x140b1d670)、utils Transform(0x140c9da40)、CritterLocomotion(0x140f8a840)。
- 无遗漏、无多余；其余全部共享 0x140002370（Default，`xor eax,eax; ret`）→ 原代码对非 17 组件**未 derive Clone**。
- 与 reconstruction 全组件 `#[derive(Clone,…)]` 标注不符——登记，不改源码。

---

## 4. cache_prev_object_transforms（0x14082bd80）复核（任务补充项）—— 已复核（O），补挖 1 处错误 + 2 处细节

- 复核结论：`+0x108` 标志两布局分支成立；chunk 表 0xa0 stride、记录表 0x48 stride、槽 0x30 取补索引均成立。
- **错误 1（已修）**：flag==0 空 chunk 判定是 **desc+0x58 == 0**，非"desc 首字段==0"（`*(*(B+0x100)+0x58 + id*0xa0)` 判零）。两章同错（ALGORITHMS §3 + drawable.rs.md §1）均已改。
- **补挖细节 1**：flag==0 分支每实例槽索引取自 chunk 专属 per-instance 描述符表 `(desc+0x50)+8 + idx*0x10`（u32，stride 0x10）；flag≠0 分支槽索引直接 = 循环计数（无描述符表）。
- **补挖细节 2**：kind 写入目标是**目标槽**的 kind 指针表（`base+0x28 + ~target*0x30`，每实例偏移 idx*4）。
- 语义（I）维持：把实例当前变换缓存进 PrevTransform 槽（velocity_only/wren 消费），两帧索引（+0x110=当前/+0x118=上一帧）。

---

## 5. NotFound 坏证据复核（任务补充项）—— 无新增（通过）

- 对 components 3 章全部证据列引用的 decomp 文件（49 个 decomp-*.txt：mod.rs 44 register + 1 serialize + drawable 1 + transient 3）逐个存在性 + 首部检查：**全部存在且非 NotFound**（内容为有效反编译，18KB 级）。
- 4 个已知 NotFound（0x140002370、0x14082bd60、0x14084cc00、0x140aa8200）已在 round 初改机器码手解，本轮复核无误。
- 补充：0x140aa8200 的"0xd8 栈帧主体"表述纠正（见 §1）——该栈帧属于相邻函数 0x140aa8210。

---

## 6. 追加闭合（groupE #14：InstancedWall::new / impl$1::from）—— 已闭合（O，机器码手解）

- **`InstancedWall::new`（0x140858800，33B）**：整体拷贝构造，`new(id, bricks: Vec<BrickTransformSsbo>（24B 按值→指针传）, writer: Option, storage: Option)` → 确认 §5 布局 `{id@0, Vec{cap@+8,ptr@+0x10,len@+0x18}, writer@+0x20, storage@+0x28}`；不深拷贝砖数据。
- **`impl$1::from`（0x140858790，37B）= `From<WallId> for BrickSourceId`**（PDB public-symbols），与 `from_wall_id`（0x140858750 = `From<ColorId> for BrickSourceId`）同逻辑：
  `id-100000` 在掩码 **0x80f 位集 {0,1,2,3,11}** 内 → 查表 0x142a953f0 返回 `0x80000000 | idx`（瞬态楼梯边，i32::MIN 高判别位）；否则原样返回 id（Wall）。
- **纠错**：旧文"wall_id∈[100000,100011] 全为瞬态楼梯边（100000+idx）"——实际仅 `{0,1,2,3,11}` 五个 id 映射瞬态边且为查表值非"100000+idx"；`with_transient_stair_edge_id`（0x140858780）常量 **0x40000000**（旧文"0x40"错）；补漏 impl$1::from 行。

---

## 7. 证据与落盘

- 新增/复用证据：`expansion-b1/decomp/decomp-0x1412e9e70.txt`、`decomp-0x140aa8070.txt`、`decomp-0x14082bd80.txt`；`building-gates/B-mesh-helpers/1412e9e70.asm.txt/.c`（RNE 基线，锁函数名权威）；`render-architecture/public-symbols.json`（锁/FunctionSystem/From 符号）；`render-architecture/public-symbols.json` + `country-core-audit/data/procs.tsv`（instanced_wall 函数名）。
- 本轮 MCP 查询：`components-geometry/jobs/xref-transient*.json`（get_xrefs_to ×3）、`jobs/xref-drop-new.json`（decompile drop / new / impl$1::from；后两者 NotFound → 机器码手解）。
- 未改 reconstruction 源码；全部差异登记于 ALGORITHMS.md / 章节 / 本文件。
