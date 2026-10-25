# A2 取证：游戏构图闭包（声明 → 图 → compile → 批次）

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra，`run_auto_analysis:false` 只读会话，`scripts/render-recovery/run-job-queued.mjs`。job/报告见本目录 `job-*.json`、`REPORT*.json`；保留伪代码见 `decomp/`(27)、`decomp2/`、`decomp3/`、`decomp4/`。
- 仅取证，未修改任何 Rust 源码、清单或既有报告。

## 交付边界

Ghidra 伪代码不是作者原 Rust 源码；PDB 签名普遍 `void()` 占位。`run_auto_analysis:false` 下 xref 完整性未知，空 xref 不等于无 caller。下文区分**观察**、**推断**、**未知**。

> **父级更正**：本文对 A2 原始报告做了三处修正/补充，见第 5 节「父级复核与更正」，其中 **A2 报告的 `0x1a+0x37→0x07` 是转录错误，实为 `→0x37`**。

## 1. 机制链条（观察）

```
游戏 pass 构造（country_core）
  ComputePass::new / DrawBuilder / UniformBinder
  → read_rg_storage_buffer / write_rg_storage_buffer / multi_draw_indexed_indirect / dispatch_indirect_with
资源解析：u64 TypeId 键 → 图内 swiss-table 查表 → u32 resource_index
  ↓
rhapsody::rg::pass_builder::PassBuilder::<UserData>::add_resource_access   0x1409dca90
  账本条目 12B {u32 resource_index, u32 access_type, u8 sync_group}
  ResourceSyncGroup::is_equivalent 0x140bece60 去重 + 访问类型合并
  ↓
PassBuilder::render（实例 0x1409dd4d0 等）
  0xe8B pass 描述符装箱 → 追加 64B 句柄进图 pass 向量 → RecordedPass<PassResources>
  ↓
RenderGraphState::compile
  ResourceInfo::calculate            0x140530430
  ResourceRegistry::populate_resources 0x140be7ba0
  CompiledRenderGraph::new           0x1402c4280   （批次步长 0x18）
  on_retire                          0x140be9420
```

**分工（观察）**：声明层 + "本帧登记哪些 pass"（即构图闭包）= **country_core 游戏码**；图容器 / 账本合并 / 资源表 / 编译 / 执行 = **rhapsody 通用码**。

## 2. `add_resource_access` 账本（父级逐行核对，`decomp/0x1409dca90.c`）

签名（Ghidra）：`FUN_1409dca90(ulonglong *param_1 /*builder*/, uint param_2 /*resource_index*/, uint param_3 /*access_code*/, undefined4 param_4 /*sync_group*/)`

- 账本基址 `param_1[1]`，**条目步长 `0xc` = 12 字节**：
  - `+0` u32 = `param_2` — resource_index
  - `+4` u32 — **`sync_group`**（见下方更正）
  - `+8` u8 — **`access_code`**（见下方更正）

> ### ⚠️ 更正（2026-10-12，由 R2 复核 + 父级机器码复核推翻本文原结论）
>
> 本文原写 `+4 = access_type`、`+8 = sync`，**这是错的，两者标反了**。
>
> **正确布局：`{+0x00 index:u32, +0x04 sync_group:u32, +0x08 access_code:u8}`。**
>
> 证据链：
> 1. `ResourceInfo::calculate`(`0x140530430`) 把 12 字节账本展开成 0x14 记录：`rec[+0x0c] = led[+0x04]`、`rec[+0x10] = (char)led[+0x08]`；
> 2. `can_be_concurrent_with`(`0x140bece70`) 读 `{+0 group:u32, +4 class:u8}`，`class = rec[+0x10] = led[+0x08]`；
> 3. 合并表比对 `existing[+8]` 与新传入的**访问码** `param_3`；
> 4. `ResourceSyncGroup::is_equivalent` 作用在 `led[+0x04]`。
> 只有 `+4 = sync_group`、`+8 = access_code` 才能让这四处同时自洽。
>
> **根因（父级已用 PE 机器码定死）**：`PassResourceAccessType::new`(`0x140b25c90`) 的机器码是 `89 D0 89 CA C3` = `mov eax,edx; mov edx,ecx; ret` —— 它是**双参数寄存器交换函数**，而 Ghidra 把它建模成单参数，于是把返回对的两个寄存器搞混，产生了 `+8 = (char)param_4` 这个**错误绑定**。
> 实参序实为 **`(builder, resource_index, access_code, sync_group)`**。
>
> **影响**：本文第 3 节的"访问码"清单仍然有效（`0x1a`/`0x24`/`0x37`/`0x02`/`0x34` 就是 access_code），但**第 2 节把 `+4` 叫 access_type、`+8` 叫 sync 的说法作废**。本文当初提出的"A2 访问码 vs A3 class 区间 `[0x28,0x37]` 矛盾"**因此不存在** —— 二者本来就是同一个字段。
>
> 另：父级当时对本文第 2 节的"逐行核对"**继承了同一个绑定错误**，一并作废。参见 `../r2-graph-barrier/findings.md`。
  → A2 的「12B {u32 index, u32 access_type, u8 sync}」**确认**。
- `param_1[8]` 指向资源访问链表：`+0x38` = 元素基址（**步长 0x10**），`+0x40` = 元素个数；每元素 `{u32 版本, u32 ?, u64 尾部索引}`。
- 去重：若 `existing.sync == new.sync` **且** `ResourceSyncGroup::is_equivalent(existing, new)` 为真 → **直接 return，不追加条目**。
- 合并（`is_equivalent` 为真时）与升级（为假时）见第 4 节。
- 不追加条目时走 `FUN_1428d9390` panic 构造（内含若干 Debug formatter 函数指针），即**debug 下的"访问冲突/不支持组合"断言**。

## 3. 访问码实测（观察，父级 grep 复核）

从 producer 函数的实际调用点取得，**不是从枚举名反推**：

| 调用点 | 地址 | 传入 access_code | 传入 sync |
|---|---|---|---|
| `ComputePass::read_rg_storage_buffer` | `0x14096dac0` | `0x1a` | `1` |
| `ComputePass::write_rg_storage_buffer` | `0x14096dd60` | `0x37` | `0` |
| `ComputePass::write_rg_storage_buffer_concurrent` | `0x14096e000` | `0x37` | 调用方 `param_5` |
| `DrawBuilder::read_rg_storage_buffer` | `0x140906150` | **`0x24`** | `1` |
| `DrawBuilder::write_rg_storage_buffer` | `0x1409067e0` | `0x37` | `0` |
| `DrawBuilder::multi_draw_indexed_indirect` | `0x140907990` | `2` | `1` |
| `dispatch_indirect_with` | `0x14160b850` | `2` | `1` |
| `pass_resource_access::{read,write,write_concurrent}_access_buffer` | `0x140970620`/`0x140975920`/`0x1409764a0` | 调用方 `param_5` | 调用方 |

**关键区分（A2 报告遗漏）**：
- `0x1a` = **compute 侧读**；`0x24` = **draw 侧读**。二者是**不同**的访问码，不能合并成"读 rg"。
- `0x37` = 写 rg（与候选 `temporal.rs` 的 `TEMPORAL_EXPORT_ACCESS = 0x37` 一致，可互相印证）。
- `0x02` = 间接绘制/派发。

## 4. 访问类型合并规则（观察，父级逐行核对）

设 `cVar6` = 账本中已有条目 access 字节，`cVar11` = 本次新 access 码：

| 已有 | 新来 | 结果 | 说明 |
|---|---|---|---|
| `0x17` | `0x1d` | `0x1e` | |
| `0x1d` | `0x17` | `0x1e` | 与上行对称 |
| `0x1a` | `0x02` | `0x02` | |
| `0x1a` | `0x37` | **`0x37`** | **A2 报告写作 `0x07`，是转录错误** |
| `0x19` 或 `0x1a` | `0x37` | **`0x37`** | A2 报告遗漏此条；同时把 sync 组清零 |

代码依据：`code_r0x0001409dcb90: if ((cVar11 == '7') && ((byte)(*pcVar13 - 0x19U) < 2))`，以及 `code_r0x0001409dcba6: *pcVar13 = '7';` —— C 字符字面量 `'7'` 是 **ASCII 0x37**，不是 `\x07`。

## 5. 编译层（观察）

- `RenderGraphState::compile` 三个实例单态化：
  - `0x140161cd0`（9204B）— 主世界帧图，内联 `render_world` + UI/编辑器
  - `0x1401647b0`（5284B）— UI/离线图，无 `render_world`
  - `0x140166510`（3338B）— present/resolve 图
- `render_loop::render` (`0x1404e8010`) 只调 #1/#2；**#3 的调用者未闭合（推断其角色）**。
- 编译后调用顺序：`ResourceInfo::calculate` (`0x140530430`) → `ResourceRegistry::populate_resources` (`0x140be7ba0`) → `CompiledRenderGraph::new` (`0x1402c4280`) → `on_retire` (`0x140be9420`)。
- 批次 = 24B `{start, end, payload}`，由 `BatchCompiler::flush_batch` (`0x140be9600`) 压入**连续 pass 区间**。与 A1 观察到的"batch 条目 0x18 字节"一致。
- 图资源声明：`RenderGraph::create_raw_resource` (`0x140be94d0`)，返回 u32 索引，描述符 0x48 字节。

## 6. country_core pass_builder 清单（观察，供后续接线）

| 接口 | 地址 |
|---|---|
| `ComputePass::new` | `0x1405876a0` / `0x1416032a0` |
| `read_rg_storage_buffer` | `0x14096dac0` |
| `write_rg_storage_buffer` | `0x14096dd60` |
| `write_rg_storage_buffer_concurrent` | `0x14096e000` |
| `pass_resource_access::read_access_buffer` | `0x140970620` |
| `pass_resource_access::write_access_buffer` | `0x140975920` |
| `pass_resource_access::write_concurrent_access_buffer` | `0x1409764a0` |
| `write_storage_buffer` | `0x141608090` |
| `read_storage_buffer` | `0x140588f60` |
| `bind_external_storage_buffer` | `0x14096e2a0` |
| `dispatch_2d` / `dispatch_with` | `0x14096ec80` / `0x14160a2e0` |
| `dispatch_indirect_with` | `0x14160b850` |
| `UniformBinder::write_uniform` | `0x14096b660` |
| `DrawBuilder::read_rg_storage_buffer` | `0x140906150` |
| `DrawBuilder::write_rg_storage_buffer` | `0x1409067e0` |
| `multi_draw_indexed_indirect` | `0x140907990` |
| `draw_stream::RasterPass::render` | `0x140868370` |
| `PassResources::new_from` | `0x140909310`（按 7 个 TypeId 从 World 取 Res） |

`0x140987dd0`（`gpu_generate_draw_lists`）是「声明 → render」的 canonical 例，pass 顺序已逐条记录于 `decomp/`。

## 7. 观察 / 推断 / 未知

**观察**：第 1–3 步机制、第 2 节账本结构、第 3 节访问码、第 4 节合并规则、第 5 节编译调用顺序与批次步长、第 6 节地址清单。

**推断**：三个 compile 实例的"角色"标注（#3 尤其；其 caller 未闭合）。批次语义由 `flush_batch` 体 + `CompiledRenderGraph::new` 偏移共同支撑，属强推断。

**未知**：
- `BatchCompiler::new` (`0x140be95e0`, 30B) 与 `pass_access_range_indices` (`0x140be9690`) 的 decompile **与** disassemble 均返回 error envelope，无法取得。
- access_type 枚举**未完整命名**（0x17/0x19/0x1d/0x1e/0x24 语义未知）。
- 闭包签名 / 捕获布局、`ResourceInfo` 与 `CompiledRenderGraph` 字段名、temporal pair 延迟未恢复。
- xref 仅返回 DATA 引用（`run_auto_analysis:false`），caller 关系未闭合。
- **未宣称恢复完整 compile ABI。**

## 8. 候选实现差异（`reconstruction/rhapsody-rebuild/src/rg/`）

- `PassBuilder::add_resource_access(resource, access_code, group)` 与 `AccessDeclaration{index, code, group}` —— **对齐较好**。
- `state.rs::compile_frame(graph, backend: impl ResourceBackend, dependency: impl AccessDependency)` **注入了 trait 对象**；原作是**单态化**并直接调具体 `ResourceInfo::calculate` / `populate_resources`。
- `pass_api.rs` 的 `Command` / `CommandBackend` 是**重建的记录适配层**；原作是 `record_batch` over `RecordedPass<PassResources>` + country_core 的 `ComputePass`/`DrawBuilder` 绘制流。
- `RenderGraph::add_pass(name)`、`buffer_ref/texture_ref/render_bound`、`PassBuilder::render(FnOnce(u64)->bool)` 均为**候选 API**，原侧未见对应形态。
- 候选**未复刻 u64 TypeId 键的资源表**（原作用类型哈希解析资源，候选按名字）。

## 9. 仍阻塞

1. `BatchCompiler::new` / `pass_access_range_indices` 无法反编译，批次构造的前段缺失。
2. access_type 枚举命名不全（0x17/0x19/0x1d/0x1e/0x24）。
3. caller 关系未闭合（xref 只有 DATA）。
4. 完整 compile ABI 未恢复。
