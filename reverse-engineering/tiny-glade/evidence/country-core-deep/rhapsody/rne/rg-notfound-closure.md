# rhapsody rg 域 NotFound 补证 rne-read 原字节（agent=rhapsody-rg-closure，2026-10-15）

程序：B patched `workspace/input/tiny-glade.exe`（SHA cf04827e…）。读法：`scripts/rne-read.py --hex <va> <n>`。
11 个 Ghidra NotFound（82B 桩）全部 rne-read 直读原字节手解，verdict 从 I/U 升级到 **O**（均为短函数）。
另核对全局计数器初值。S_PUB32 权威名以 `evidence/render-architecture/public-symbols.json` 为准（ICF 折叠区以 S_PUB32 权威）。

## 1. BatchCompiler::new — 0x140be95e0（O）
```
0x140be95e0  4889c8 48c70100000000 48c7410808000000 0f57c0 0f114110 0f114119 c3
```
手解：`mov rax,rcx; mov qword[rcx],0; mov qword[rcx+0x8],8; xorps xmm0; movups[rcx+0x10]; movups[rcx+0x19]; ret`
- +0x00=0（batches Vec cap，cap-first）、+0x08=**8**（空 Vec 悬垂指针=align_of::<Batch>，Batch 0x18B align 8）、+0x10=0（len）
- +0x18（current_range.start）=0、+0x20（current_range.end）=0（xmm 覆盖 0x10..0x28）
- +0x28（current_export_barriers）=0
- 与 ALGORITHMS §1.2 Batch/BatchCompiler 布局一致；紧邻 0x140be9600 为 flush_batch 正文（`cmp [rcx+0x18],rdx; je` 早退）。
- S_PUB32：`rhapsody::rg::compiled::BatchCompiler::new`（1 符号，非折叠）。

## 2. pass_access_range_indices — 0x140be9690（O，身份函数；用途由消费端定）
```
0x140be9690  89c8 89d2 c3
```
手解：`mov eax,ecx; mov edx,edx; ret` —— 身份/passthrough（返回输入 u32）。
- **消费端证据**：add_pre_pass_barriers（0x1402c3d00，本轮补反编译）内 `func_0x000140be9690(*(uint*)(*(param_1+0x68)+param_2*8))` —— 传入 `pass_access_ranges[pass].start`，返回值作为访问循环起始下标；`pass_access_ranges Vec<{u32 start,u32 end}>` 在 CompiledRenderGraph+0x68(ptr)/+0x70(len)。
- 作用：取 pass 访问范围边界（编译器优化为恒等）；无 hash/运算。
- S_PUB32：`rhapsody::rg::compiled::pass_access_range_indices`（1 符号，非折叠）。

## 3. PassPgoNameId::from_name — 0x14067dd80（O；**原 hash 算法 = 无 hash，取 &str 指针为 ID**）
```
0x14067dd80  4889c8 c3
```
手解：`mov rax,rcx; ret` —— 身份函数。
- **ICF 折叠区**：S_PUB32 同址折叠 ≥8 符号：`<ComputePipeline as Deref>::deref`、`<TypedStorageBuffer<T> as StorageBufferDyn>::buffer`×N、`ImageFormationConfigs::initial`、`PassPgoNameId::from_name` 等——全部为取字段/Deref/from 恒等实现。`mov rax,rcx;ret` 即共享体。
- **消费端证据**：record_batch（0x1402c1d40）贪心分块内 `func_0x00014067dd80(pass+0x18, pass+0x20)` = from_name(&str 指针+长度) → 返回指针作为 PassPgoNameId。
- **纠错**：ALGORITHMS §2.2"原 hash 算法 U（rebuild FNV-1a 是候选）"不成立——原版 PassPgoNameId=名称指针（零成本转换），不做 hash。真正 hash 在 `PassPgoStats::duration_ns`（0x140bea1e0）的 hashbrown 查找里（`FUN_1405b0d70`=hash_one）。rebuild executor.rs FNV-1a 是偏差（非原行为）。

## 4. PassPgoDuration::new — 0x140be9c70（O，平凡透传构造）
```
0x140be9c70  4c89c2 4889c8 c3
```
手解：`mov rdx,r8; mov rax,rcx; ret` —— 无计算透传构造（newtype 包装）。
- S_PUB32：`rhapsody::rg::executor::PassPgoDuration::new`（1 符号，非折叠）。

## 5. RenderPassApi::end_render_pass — 0x1416f7850（O）
```
0x1416f7850  488b01 488b5108 488b4848 ffa228040000
```
手解：`mov rax,[rcx]; mov rdx,[rcx+8]; mov rcx,[rax+0x48]; jmp qword[rdx+0x428]`
- api+0 → api 上下文对象、api+8 → device；cb 取 [api 上下文]+0x48；**尾调 device vtable+0x428**（vkCmdEndRenderPass）。
- 与 begin_render_pass（0x1416f6e90）的 vtable 系对照一致。
- S_PUB32：`rhapsody::rg::pass_api::RenderPassApi::end_render_pass`（1 符号，非折叠）。

## 6. RenderPassApi::set_line_width — 0x1416f7960（O；**device+0xad9 门控为原机器码**）
```
0x1416f7960  488b4108 80b8d90a000000 7411 488b09 488b4948 488b80f0020000 48ffe0 c3
```
手解：`mov rax,[rcx+8]; cmp byte[rax+0xad9],0; je ret; mov rcx,[rcx]; mov rcx,[rcx+0x48]; mov rax,[rax+0x2f0]; jmp rax`
- device+0xad9==0 → 直接 `ret`（no-op）；否则取 cb 后 **尾调 device vtable+0x2f0**（vkCmdSetLineWidth），rdx=线宽原样传。
- **纠错**：ALGORITHMS §2.6"device+0xad9 门控（rebuild 推断）"→ 原机器码证实 **O**。
- S_PUB32：`rhapsody::rg::pass_api::RenderPassApi::set_line_width`（1 符号，非折叠）。

## 7. PassResourceRef::resource_index — 0x140bece40（O）
```
0x140bece40  8b01 c3
```
手解：`mov eax,[rcx]; ret` —— 取 this+0 的 u32（资源号）。
- ICF 折叠 3 符号：`utils::loop_range::LoopIdx::raw_idx`、`AtomicU32::unsync_load`、`PassResourceRef::resource_index`——同型 load 恒等。
- S_PUB32 权威：`rhapsody::rg::recorder::PassResourceRef::resource_index`。

## 8. ResourceSyncGroup::new / Default::default — 0x140bece50（O；**全局计数器 @0x14362c230 初值=2，递增 1**）
```
0x140bece50  b801000000 f00fc105d3f3a302 c3
```
手解：`mov eax,1; lock xadd [rip+0x2a3f3d3],eax; ret` —— 对**全局 AtomicU32** fetch_add(1) 返旧值。
- RIP-relative 目标精确计算：0x140bece55+8+0x2a3f3d3 = **0x14362c230**；rne-read 该地址文件字节 = `02 00 00 00 00 00 00 00` → **初值 2**（.data）。
- 语义：组号从 2 起（0/1 保留给 NONE/单访问），每次 +1。
- S_PUB32：`ResourceSyncGroup::new` 与 `ResourceSyncGroup as Default::default` **2 符号折叠同址**——new 与 default 同体（都取全局计数）。ALGORITHMS §2.9"new"归属成立，行为 O。
- rebuild `access_ledger.rs` 注释"原 0x140bece50 返回 fetch_add 旧值"一致 ✓；rebuild 用实例 counter（差异：原为全局）。

## 9. ResourceSyncGroup::is_equivalent — 0x140bece60（O）
```
0x140bece60  8b01 3b02 0f94c0 c3
```
手解：`mov eax,[rcx]; cmp eax,[rdx]; setz al; ret` —— u32 相等。
- S_PUB32：`rhapsody::rg::recorder::ResourceSyncGroup::is_equivalent`（1 符号，非折叠）。

## 10. PassResourceAccessType::new — 0x140b25c90（O，平凡构造）
```
0x140b25c90  89d0 89ca c3
```
手解：`mov eax,edx; mov edx,ecx; ret` —— 无计算构造（透明 newtype）。
- ICF 折叠 2 符号：`DecoratorLibKey::new`、`PassResourceAccessType::new`——同型平凡构造。
- 布局（由 can_be_concurrent_with 0x140bece70 证实）：`{u32 sync_group @0, u8 access_code @4}`。

## 11. ResourceHandle::raw — 0x140833250（O）
```
0x140833250  89c8 c3
```
手解：`mov eax,ecx; ret` —— 恒等（透明句柄取底层序号）。
- **确证 rebuild 注释"原 raw 入口直接返回 ECX"**。
- ICF 折叠 11 符号（`LazyTile::from_index`、`RaycastMeshKey From`、`MeshAtlasPart::new`、`ResourceHandle::raw` 等）——全为恒等。
- S_PUB32 权威：`rhapsody::rg::resource::ResourceHandle::raw`。

## 12. 补充核对：PassPgoStats::duration_ns — 0x140bea1e0（O，**实为 contains_key 返回 bool**）
- 非 NotFound（已有 decomp），但本轮确证行为：`if (param_1[3]!=0) { hash_one(param_1+4, &key); hashbrown probe; return 1/0; }` —— 返回 **bool**（含不含），不是时长 u64。
- **PDB 名与行为错位**：`duration_ns` 实为 `contains_key`。record_batch 分块用它判"该 pass 是否有 PGO 记录"。ALGORITHMS §2.2"未命中返 0（分块按 0 计）"措辞需按 bool 语义修正。
- S_PUB32：`rhapsody::rg::executor::PassPgoStats::duration_ns`。

## 13. 关联解析（S_PUB32，供集成轮）
- `CompiledRenderGraph::add_pre_pass_barriers` = **0x1402c3d00**（真实编译函数，非纯 inline）。
- `<Vec<T> as SpecFromIter<T,I>>::from_iter` = **0x1402bf880**（见 ALGORITHMS §4：+0x200 初始过渡列表由 from_iter 构建；**审查轮"compute_batches 指向 0x1402bf880"假设不成立**）。
- `SynchronizationScope1::new` = **0x140bd6650**（与 Default::default 折叠同址）；`image_barrier`=0x140bd66a0、`buffer_barrier`=0x140bd6810、`reset`=0x140bd6950、`is_empty`=0x140bd6980、`issue_pipeline_barrier`=0x140bee030、`ImageBarrier::new`=0x140bd65e0、`BufferBarrier::new`=0x140bd6610。
- `Buffer as ImportExportToRenderGraph::export` = **0x140bd5120**（闭合 U 卡点#9 归属）。
- `compute_batches` / `gather_batch_preamble_barriers` / `record_postamble` 无编译符号（S_PUB32 无）——纯 inline，体在 record_batch 0x1402c1d40。
- `validate_read_access` ×2 无编译符号——仍保持"inline no-op（rebuild 断言候选）"，本轮无新证据可升级。

## 证据文件
- 本轮新增反编译（RE-MCP dbid `tg_rh_rg_cls`，job `domains/rg/rg-closure-decomp.json` → `rg-closure-out.json`）：
  `domains/rg/decomp/decomp-0x1402c3d00.txt`（add_pre_pass_barriers 8.8KB）、`decomp-0x140bd6470.txt`（into_sampled_rg_texture 2.6KB）、`decomp-0x140bd66a0.txt`（image_barrier 3.2KB）、`decomp-0x140bd6810.txt`（buffer_barrier 2.4KB）、`decomp-0x140bee030.txt`（issue_pipeline_barrier 0.5KB）。
- 0x140bd6650 / 0x140bd6950 / 0x140bd6980 / 0x140bd65e0 / 0x140bd6610 在 Ghidra 仍 NotFound（本轮 rne-read 手解，见 ALGORITHMS §4 布局推导）。
