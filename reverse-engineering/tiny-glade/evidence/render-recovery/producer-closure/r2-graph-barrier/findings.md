# R2 复核：构图闭包 + barrier/recorder

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`。产物：`job-a..g-*.json`、`REPORT-a..g.json`、`decomp/`（44 份）。
- 仅取证，未修改任何游戏代码。

## 0. 工具面事实（影响旧证据可信度）

- **`disassemble_function` 在本配置恒返回 0 条指令**（函数表有 116,272 项，但指令未持久化；先 decompile 再 disassemble 仍为 0）。⇒ **任何"看到汇编"的断言都不可由该工具复现**；A3 引用的 `add $-0x28,%cl` 只能由 decompile 复核。
- `get_xrefs_to` 只回 DATA 引用（`0x1409dca90` 仅 1 条）⇒ **call xref 不可用**。R2 改用离线静态调用图 `workgraph/calls.json`（450 函数）闭合调用者。

> **父级方案**：本项目的汇编类断言（如 `lock xadd`、寄存器交换、指令立即数）**改用直接读 PE 镜像**复核，见第 6 节。该方法不依赖 Ghidra。

## 1. 核心结论：A2/A3 的"矛盾"不存在，是 A2 的字段标反了

### 结论
**账本 12 字节的真实布局是 `{+0x00 index:u32, +0x04 sync_group:u32, +0x08 access_code:u8}`。**
**A2（及父级转述）把 `+4` 与 `+8` 标反了。**

### 证据链
- `ResourceInfo::calculate`(`0x140530430`) 把 12 字节账本展开成 0x14 记录：`rec[+0x0c] = led[+0x04]`、`rec[+0x10] = (char)led[+0x08]`。
- `can_be_concurrent_with`(`0x140bece70`) 读的是 `{+0 group:u32, +4 class:u8}`，其中 `class = rec[+0x10] = led[+0x08]`。
- 合并表（`add_resource_access` 内）比对的是 `existing[+8]` 与**新传来的访问码** `param_3`。
- `ResourceSyncGroup::is_equivalent` 作用在 `led[+0x04]`（= 与 `param_4` 比较）。

⇒ 只有 `+4 = sync_group`、`+8 = access_code` 才能让这三处同时自洽。

### 根因（父级已用机器码定死）
`PassResourceAccessType::new` 是 **2 参数交换函数**，而 Ghidra 把它建模成 1 参数，于是把返回对的两个寄存器搞混，产生了 `+8 = (char)param_4` 这个**错误的绑定**。
实参序为 **`(builder, resource_index, access_code, sync_group)`**（wrapper `0x140970620`/`0x140975920` 的第 5 参即访问码）。

### 于是"矛盾"消失
四类访问码 `0x1a`/`0x24`/`0x02`/`0x37` 与 A3 的 classify 是**同一个字段**：
`classify(v) = (v - 0x28) < 0x10` 挑出 `[0x28,0x37]` 内的码（目前含 `0x37` 与 C1 新发现的 `0x34`）——**只有这些码需要"非零且相等 group"才可并发**。
这**正是** `write_rg_storage_buffer_concurrent`（照传 `0x37` + 调用方 group）的用途。

**残余不确定**：call-xref 不可用，**无法排除尚未恢复的游戏侧调用者**。

## 2. 访问码语义（证据支持者）

| 码 | 语义 |
|---|---|
| `0x1a` | compute 侧 storage 读 |
| `0x24` | draw 侧 storage 读（A2 的区分成立） |
| `0x37` | storage 写（= 候选 temporal-r5 的 EXPORT access） |
| `0x34` | （C1 观测，`clear_rg_buffer`） |
| `0x02` | 间接 draw/dispatch |
| `0x17`/`0x19`/`0x1d`/`0x1e` | **仅出现在合并表，语义 unknown，不命名** |

**确认**：`0x1a ⊕ 0x37 → '7'` 是 ASCII **`0x37`**（**非 `0x07`**，A2 的转录错误）。合并表逐行复核无误，另含 `{0x19, 0x1a} ⊕ 0x37 → 0x37` **且 group 清零**。

## 3. 两个"不可取"的函数（只给 size 与调用边）

| 函数 | 地址 | 大小 | caller |
|---|---|---|---|
| `BatchCompiler::new` | `0x140be95e0` | 30 B | `GraphExecutionRecorder::execute_compiled` `0x1400fa190` @ `0x1400fa3c6` |
| `pass_access_range_indices` | `0x140be9690` | **5 B** | `add_pre_pass_barriers` `0x1402c3d00` @ `0x1402c3d36` |

**函数体不恢复**（Ghidra 未建函数、无别名）。

## 4. 逐字段复核

1. 账本 12 B = `{+0 index:u32, +4 sync_group:u32, +8 access_code:u8}`（**A2 的错标已纠**）。
2. `param_1[8]` 链表元素 **0x10** 字节：`create_raw_resource`(`0x140be94d0`) 初始化为 `{+0 = 0xffffffff, +4 = 0, +8 u64 = 0}`。
3. 记录 stride **0x14**：`+0x04 = prev`（`0xffffffff` 哨兵）、**`+0x08 = next`（前向链，`ResourceInfo::calculate` 置 `0xffffffff`）**、`+0x0c = sync group`、`+0x10 = access code`。→ **A2/A3 未答的 `+0x08` 已答 = 前向链。**
4. `SynchronizationScope1`（0x50 字节）：三向量按 `vkCmdPipelineBarrier` 数组序 = MemBarrier（元素 0x18，长度 `+0x00/08/10`）、BufferBarrier（0x38，`+0x18/20/28`）、ImageBarrier（0x48，`+0x30/38/40`）；`+0x48` u64 = `{src_stage, dst_stage}`，被 **OR 累加**，初值 `0x0000200000000001` → src = 1 (`TOP_OF_PIPE`)、dst = 0x2000，**其余位义 unknown**。
5. `raw::barrier` **恰 10 个**（`functions.json` 复核），**A3 已全列** —— 所以此前"剩 8 个未读"的前提**不成立**。
   `issue_pipeline_barrier`(`0x140bee030`) = vtable `+0x3e0` 调 `vkCmdPipelineBarrier`（10 参）；`issue_full_pipeline_barrier`(`0x140bee090`) = `src = dst = 0x10000`。
6. `rg::pass_builder` 剩 5 个 = `PassBuilder::render` 的 5 个实例（`0x140533d90`/`0x1409dcd10`/`0x1409dcfd0`/`0x1409dd240`/`0x1409dd4d0`），另有**第二个** `add_resource_access` `0x140533b10`（与 `0x1409dca90` 同构）。
7. `rg::recorder` 剩 11 个均为 22–157 B 的 `impl$1::render<closure>` thunk。
8. **`RenderGraphState::compile` 实例 `0x140166510` 的 caller 已闭合 = `GraphExecutionRecorder::execute`(`0x1400fb5b0`) @ `0x1400fb609`**；#1/#2 在 450 函数集内无调用者。
   ⇒ 三实例的"角色"标注**降级为 low-confidence / unknown**。

## 5. 仍未知

- `disassembly` 不可用（见第 0 节）；
- 游戏侧尚未恢复的 caller；
- `0x17`/`0x19`/`0x1d`/`0x1e` 的语义、掩码 `+0x48` 的其余位义；
- 两个 C 类函数的函数体。

## 6. 父级独立复核（直接读 PE，不依赖 Ghidra）

R2 的核心论断由父级**从 PE 镜像直接读取机器码**验证（方法见 `r4-descriptor-shader-swapchain/findings.md` 第 1 节）：

| 函数 | 地址 | 机器码 | 解码 | 结论 |
|---|---|---|---|---|
| `PassResourceAccessType::new` | `0x140b25c90` | `89 D0 89 CA C3` | `mov eax,edx; mov edx,ecx; ret` | **双参寄存器交换** —— 证实 Ghidra 的 1 参建模是错位根因 |
| `ResourceSyncGroup::new` | `0x140bece50` | `B8 01 00 00 00` `F0 0F C1 05 …` `C3` | `mov eax,1; lock xadd [rip+…],eax; ret` | **全局原子 `fetch_add(1)`** —— A3 确认 |
| `is_equivalent` | `0x140bece60` | `8B 01 3B 02 0F 94 C0 C3` | `mov eax,[rcx]; cmp eax,[rdx]; sete al; ret` | **只比较 u32**（workgraph 的 "size 8" 是代码长度，非比较宽度）—— A3 确认 |
| `can_be_concurrent_with` | `0x140bece70` | 开头 `0F B6 59 04` | `movzx ebx, byte ptr [rcx+4]` | **读首参 `+4` 的字节** = class —— 与 R2 一致 |
| `PassResourceRef::resource_index` | `0x140bece40` | `8B 01 C3` | `mov eax,[rcx]; ret` | 取 `+0` —— A3 确认 |

⇒ **`{+4 = sync_group, +8 = access_code}` 由机器码闭合**，A2 的字段互换结论成立。
