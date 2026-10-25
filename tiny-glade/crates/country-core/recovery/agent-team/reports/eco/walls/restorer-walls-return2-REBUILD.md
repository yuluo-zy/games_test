# REBUILD — walls 域 11 函数逐函数判等（restorer-walls-return2，2026-10-17）

> 协议：`planning/2026-10-16-logic-verify-protocol.md`。方法：**只以 fresh 反编译为准**
> （`reverse-operator.py ghidra --worker shared`，`fields=decompiled_code,callees`；disasm
> `disassemble_function` 三个函数）。本轮 fresh 证据落盘
> `planning/impl-walls-return2/decomp/`（batch1-fresh/result、batch1-disasm-result、
> batch2/3-fresh/result）。未采信此前 restorer/reviewer 描述；每函数以本次 fresh 反编译/反汇编定案。

## 结论概览

| VA | 函数 | verdict |
|---|---|---|
| 0x140a1f590 | DecoratorStorage::on_wall_split | **判等**（disasm 逐偏移定案） |
| 0x14155b170 | Decorators::merge_with\<DecoMoveToStorage\> | **判等**（三循环逐条；d2s insert/update 分支差异登记） |
| 0x140a1ea40 | DecoratorStorage::on_walls_merged | **判等** |
| 0x140a20d70 | restore_snapshot / take_snapshot | **判等-注意**（核心还原语义已重建；0x28B/0x16B 节格式简化登记） |
| 0x1411ff330 | ShapeParameters::contains_shape | **判等**（6/6/4/4 采样逐条） |
| 0x1411ff680 | ShapeParameters::expand | **判等**（圆车道保留原值） |
| 0x140a6ad00 | snapping_maintain | **判等-注意**（全状态发布 ✓；未载入字段原生发布寄存器垃圾，不可复现） |
| 0x140a6b5e0 | snapping_to_segments | **判等**（本轮补 cursor.tag 门；排序取最近 + SQRT<1.5 ✓） |
| 0x140aa5a10 | query_with_aabb_clamped | **判等**（本轮重建：排序 + 同墙去重 + 段最近点候选） |
| 0x140a82b20/2b40/2d50 | CompoundEditBuilder new/next/build | **判等**（无 started/is_empty/None） |

改动文件（walls 域内）：`systems/wall/wall_state/wall_spatial_hash.rs`、
`systems/wall/snapping/snapping_to_segments.rs`、`systems/wall/shapes/shape_placement.rs`（仅注释）。
`cargo check -p country-core --lib` 通过；`cargo test -p country-core --lib wall` 86 passed、
decorator_storage 13、valid_enclosure 6、snapping 8、decorator 6、wall_history 4。

---

## 1. on_wall_split 0x140a1f590 — 判等（disasm 定案）

batch1-disasm `0-on_wall_split-disasm.txt`（715 指令）逐偏移：

- **immutable 门**：`CMP byte ptr [RCX+0x1b0],0x0; JNZ panic` ✓。
- **per_wall.remove(wall)**：`RawTable::remove_entry<WallId,Decorators>`，未中跳过（返回）。✓
- **首块 removed_ids 注册表**：`rustc_entry<Addr,HashSet> + extend<DecoratorId,()>`（原墙地址键）。✓
- **主循环 break 字节 = DecoratorInfo.state @+0x47**：`MOVZX EAX,byte [RDI+idx*0x50-1];
  CMP AL,0x2; JZ break`（id 读于 `[RAX-0x50]`，break 字节 = entry+0x4f = DecoratorInfo+0x47）——
  与 Rust `info.state == 2` ✓。
- **d2s remove**：`CMP dword [RBP+0x1a8],0x3; JZ panic`（miss==3 → panic）✓。
- **侧判别字节 = DecoratorInfo.dst_tag @+0x22**：`MOVZX EAX,byte [RBP+0x52]`（拷贝块基 RBP+0x30，
  +0x52-0x30 = 0x22）→ `LEA ECX,[RAX-2]; DEC RAX; CMP CL,3; CMOVNC RAX,0` =
  `(b-2)<=2 ? b-1 : 0` ✓ 与 Rust `side_from_state`。
- **侧 0**：`get_approx_u_from_pos`（点 = `MOVSS XMM1,[RBP+0x44]`=dst_payload 高 32、
  `MOVD XMM2,[RBP+0x4c]`=u18 高 32）→ `MAXSS(0,u)` + `MINSS(1,·)` clamp[0,1] → 子段
  `{WallId@+0, low@+8, high@+0xc}` stride 0x10 包含搜索（`UCOMISS u,low/high`）→ 命中
  **插入子墙桶**（`Decorators::insert`，经 `LAB_140a20119`）；未命中 → 注册表。✓
- **侧 1**：`rustc_entry + HashSet::insert<DecoratorId,()>`（注册表，原墙地址键）✓。
- **侧 ≥2**：`JNZ 0x140a202a3`（unreachable panic）✓。

Rust body（decorator_storage.rs:740）与上述逐条对应，**无分歧**。测试 4 条（侧0 直插/
侧0 未中/state2 break/侧23 panic）全过。

## 2. merge_with\<DecoMoveToStorage\> 0x14155b170 — 判等

batch1-decomp 三循环 + disasm `1-merge_with-disasm.txt` 复核：

- **循环①**（source.removed_ids，decomp 139-266）：目标内嵌 map 无此 id（hashbrown 探针 miss）
  → `HashMap::insert<DecoratorId,()>`（目标 modified_ids）✓。
- **循环②**（目标内嵌 map，decomp 287-365 / disasm 14155b400-14155b48f）：逐 id
  d2s `remove_entry`（`CMP [RBP+0xb8],3; JZ panic` miss==3）+ d2s insert（
  `CMP RAX,3; JNZ panic` 冲突）✓。
- **循环③**（source 内嵌 map，disasm 14155b53d-14155b75e）：break 字节 @+0x47（
  `MOVZX EAX,[R14+idx*0x50-1]; CMP AL,0x2; JZ`）→ d2s remove（miss==3 panic）→
  removed_ids remove_entry → modified_ids insert → 内嵌 map insert（`CALL 0x1411fbfa0`）→
  **d2s 分支** `CMP byte [RBP+0xf7],0x2; JNZ → d2s update (0x140a1b5a0)` / `JZ` → d2s insert。
  `RBP+0xf7` = 内嵌 map insert 返回旧 DecoratorInfo 的 +0x47（sret 缓冲 RBP+0xb0，+0x47）
  = **旧值 state==2 → d2s.insert，否则 d2s.update**。

**分歧登记（判等-注意）**：Rust 循环③ 恒 `d2s.update`。原生旧值 state==2 才 d2s.insert；
但该 id 刚在循环③ 内 d2s.remove，insert 必不冲突，最终 d2s[id]=addr 与 update 等价——
**净效果一致**，登记不改。

Rust body（decorator_storage.rs:257）三循环逐条对应，**判等**。

## 3. on_walls_merged 0x140a1ea40 — 判等

batch1-decomp 逐块：

- immutable 门 ✓；`remove_entry(a)` → 若未中（`local_3c0==0`）→ 用 b；b 也未中 → **return** ✓。
- **双源都在**（decomp 136-152）：memcpy 两源 0x88 → `merge_with<DecoMoveToStorage>`（
  **d2s 重索引内置于 merge_with**）→ 结果入 per_wall[merged]（decomp 260-528 探针 + 新建 +
  drop 旧值 + memcpy 0x90）✓。
- **单源**（decomp 154-254）：逐装饰 d2s remove（miss==3 panic）+ d2s insert（冲突 panic），
  源桶原样迁入 merged ✓（= Rust `reindex_to_storage + clone`）。
- **重复 id 跨源**：走 merge_with 循环②/③（remove 后 insert/update），不 panic——Rust
  现亦直接调 merge_with（不再对第二源 assert_exists）✓。

Rust body（decorator_storage.rs:681）**判等**；测试 `on_walls_merged_migrates_and_updates_index` 过。

## 4. restore_snapshot / take_snapshot 0x140a20d70 — 判等-注意

batch1-decomp（607B）：

- **前置 6 断言** ✓（+0x1b0 immutable / +0x18 per_wall / +0x90 terrain / +0x120 stairs /
  +0x48 removed / +0x78 d2s）。
- **逐 0x28B 记录**（decomp 150-521）：`get_or_insert_default` → **状态判别跳表**
  （`(bVar30-0x38) 夹 6`，`&DAT_142ac9bdc` switch，逐状态读 0x16B 子记录字段装配
  DecoratorInfo）→ `removed_ids.remove_entry + modified_ids.insert + HashMap::insert<Id,
  DecoratorInfo>`（**恢复 DecoratorInfo 入内嵌 map**）→ 状态字节分支（`cStack_99 != 2` →
  d2s 探针 update；==2 → d2s insert 冲突 panic）。

**已重建**：Rust restore_snapshot（decorator_storage.rs:986）现含 removed_ids.remove +
modified_ids.insert + 内嵌 map insert(info) + d2s.insert —— LOGIC §1.4「不恢复
DecoratorInfo」已闭合。

**登记（判等-注意）**：① 0x28B/0x16B 节格式（状态判别跳表装配）未逐槽重建——Rust
`DecoratorAddrSnapshot` 携带完整 DecoratorInfo（注释已登记）；② 状态字节 d2s insert-vs-update
分支简化为恒 insert（前置 d2s 空断言下无冲突）；③ 原生另有 in_R8 指向的 u32 计数器副作用
`*counter = max(*counter, id+1)`（每恢复装饰更新），Rust 无该参数/调用方，登记不接线。
take_snapshot 断言返回空 ✓（捕获面未接线，前置断言保证空）。

## 5. contains_shape 0x1411ff330 — 判等

batch1-decomp 四路计数：自=圆/他=圆 **6×get_pos_at_u + 6×contains_point**；自=圆/他=矩形
**4×get_point + 4×contains_point**；自=矩形/他=圆 6×；自=矩形/他=矩形 4×。Rust
`Shape::contains_shape`（valid_enclosure/mod.rs:103）采样集 `CIRCLE_CONTAINS_SAMPLES`（
含 1/2）与 `(0..4)` 逐条对应 ✓。**判等**；测试 4 条过。

## 6. expand 0x1411ff680 — 判等

batch1-decomp 位运算：`iVar1==0`（圆）→ 仅 `fVar4(+0xc) += amount`；`+0x14/+0x18` 经
`CONCAT44(~uVar7&(2a+fVar5)|fVar5&uVar7)`，uVar7 圆时全 1 → **保留原值**；矩形（uVar7=0）
→ `+0x14/+0x18 = 原值 + 2a`。Rust `expand`（valid_enclosure/mod.rs:319）圆车道保留 +
矩形加 2a ✓，与 LOGIC §1.6「覆写」解读相反（fresh 证实为加）。**判等**；测试过。

## 7. snapping_maintain 0x140a6ad00 — 判等-注意

batch1-decomp：按 mode 跳表（case0/1/2/3）读 snapped/snap/wall/end，**整体发布到
+0x20..0x38**（`pbVar2[0x20]=mode、[0x21]=snapped、[0x24]=snap8B、[0x2c]=snap4B、
[0x30]=wall、[0x38]=end`）。Rust `snapping_maintain` 现发布全状态副本（SnappingModePublished
布局 +0x20..0x38 ✓）。**登记**：原生 mode 0/1 未载入的字段发布寄存器垃圾（`in_XMM*`/
`in_R10B` 未初始化，不可复现）；Rust 发布已定义值（安全超集，snapping_audio 只比模式字节，
等价）。另有 `*(u32*)[mode+0x10] = [mode+0x1c]`（bevy Events 发布计数器，胶水）。

## 8. snapping_to_segments 0x140a6b5e0 — 判等（本轮补 cursor.tag 门 + 段最近点）

batch1-decomp + disasm `2-snapping_to_segments-disasm.txt` + 闭包 fresh（0x140b16500）：

- mode==1 门 ✓；`from_iter<SnapProposal, filter_map<closest_segment_to_snap_to::closure_env_0>>`
  （0x1412fba80；闭包 0x140b16500：public 探针 + `get_approx_coord_from_pos<Vec3,
  Thresholded3D>` + `lerp(point[i], point[i+1], t)` 作 **snap 点**）→ `sort_by`（
  smallsort/driftsort）→ 取 `*local_a8`（**排序后最近**）→ `SQRT(dx²+dy²) < 1.5` 才写
  mode=3。
- **参考点 = cursor.tag 命中 ? {pos.x, pos.z} : {0,0}**（disasm `TEST byte [R8],0x1; JZ
  →{0,0}`；`MOV [RBP+0x18],qword [R8+0x4]`=pos.x、`MOV [RBP+0x20],dword [R8+0xc]`=pos.z）。

**本轮改动**：
- `snapping_to_segments.rs`：参考点加 **cursor.tag & 1 门**（`(cursor.tag & 1)!=0 ? (pos.x,
  pos.z) : (0.0,0.0)`）；**mode.snap 地形高度改在候选(snap)点采样**
  （`sample_at_world_xz(candidate[0], candidate[1])`，fresh `x0y` swizzle 0x140c9cdc0 +
  `sample_at_world_xz` 0x1409e9640 以候选点采样——Rust 原在游标采样）。
- `wall_spatial_hash.rs query_with_aabb_clamped`：候选从**段端点**改为**段到 AABB 中心最近点**
  （`segment_closest_point_to`，新 helper）——消费端 SnapProposal 即段最近点（0x140b16500），
  段为简化曲线直边，最近点 = 原生 snap 点；排序键 `endpoint_distance2_to(center, 最近点)`
  = 段距离²；同墙去重 keep 最近保留。shape_placement 消费同一返回语义（仅注释更新）。

排序后取首个 <1.5 与原生「只判最近候选」**功能等价**（升序下首 <1.5 即最近；最近 ≥1.5 则
全 ≥1.5 不写 mode）✓。

**登记**：① in_progress.wall None → Rust return vs 原生 unwrap_failed panic（系统输入边，
不改）；② mode.snapped 写 `byte_0x29e()!=0`（bool）vs 原生写原始字节（0/1 下等价）；
③ `*(u32*)[mode+0x10] = [mode+0x1c]` bevy 事件发布计数器胶水。测试 1 条过。

## 9. query_with_aabb_clamped 0x140aa5a10 — 判等（本轮重建）

batch1-decomp（与 query_with_circle 0x140aa5940 同骨架）：`get_cells_from_aabb<closure>`
收集 **WallSegment 指针** → `1<len<0x15` smallsort / ≥0x15 ipnsort（`sort_unstable_by<
WallSegment, query_with_circle::closure_env_1>`）→ **相邻同墙去重 keep 最近**（
`*(seg+0x30)` wall id 比较）。Rust 现：收集段最近点 → sort_unstable_by（到 AABB 中心距离）
→ 同墙去重（keep 最近）。**已重建**端点→最近点（见 §8），结构与 fresh 对应。

## 10. CompoundEditBuilder new/next/build 0x140a82b20/2b40/2d50 — 判等

batch1-decomp 三件：
- `new_edit_sequence`（27B）：`*RCX=0; [RCX+8]=8; [RCX+0x10]=0`（Vec{ptr=0,cap=8,len=0}，
  **无 started 标志**）。Rust `Vec::with_capacity(8)`（急切分配 vs 原生 ptr=0 惰性；cap/len
  语义等价）✓。
- `next`（291B）：alloc 0xf8 + memcpy 单条 edit → 建新组（alloc 0x108）→ `len==cap` grow_one
  → push（**无 is_empty 门**）。Rust `self.groups.push(vec![edit])` ✓。
- `build`（150B）：建 Vec{ptr=0,cap=8,len=0} → `from_iter<ConcurrentEdits, Map<Vec<WallHistoryEdit>
  →Vec<Arc<WallHistoryEdit>>>>` 收集（**无 None/空判据**）。Rust `WallCompoundHistoryEdit
  { groups: self.groups }` ✓。

**登记（判等-注意）**：原生 build 把每组 edit 包 **Arc\<WallHistoryEdit\>**（history 域
ConcurrentEdits 类型）；Rust `groups: Vec<Vec<WallHistoryEdit>>` 未 Arc 包装（表示差异，
group 序保留，测试 `concurrent_edits_groups_preserve_insertion_order` 过）。

---

## 自检

- `cargo check -p country-core --lib` 通过（仅既有 non_snake_case/unused 警告，非本批）。
- `cargo test -p country-core --lib`：wall 86、decorator_storage 13、valid_enclosure 6、
  snapping 8、decorator 6、wall_history 4 全过。
- 改动文件 3（walls 域）；未改他模块业务；共用只读 `--worker shared`，本任务无需写库。
- fresh 证据：`planning/impl-walls-return2/decomp/batch1-fresh.json→batch1-result.json`（13 函数）、
  `batch1-disasm.json→batch1-disasm-result.json`（on_wall_split/merge_with/snapping_to_segments）、
  `batch2-fresh.json→batch2-result.json`（from_iter 0x1412fba80 + find）、
  `batch3-fresh.json→batch3-result.json`（闭包 0x140b16500）。
