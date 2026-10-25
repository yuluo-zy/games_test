# A5 取证：descriptor 分配 + shader 反射→布局→管线

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra（`run_auto_analysis:false`）+ `run-job-queued.mjs`。job/报告见 `job.json`/`REPORT.json`、`job2.json`/`REPORT2.json`，`xrefs.json`；保留伪代码见本目录 `0x*.c` 与 `batch05-0x*.c`（从 batch-05 复制，未改动）。
- 仅取证，未修改任何 Rust 源码或依赖。

## 交付边界

Ghidra 伪代码不是作者原 Rust 源码；PDB 签名普遍 `void()` 占位。**本库 `disassemble` 返回 0 指令**，因此**没有机器码级核对**；跳表分支无法坐实。`run_auto_analysis:false` 下 xref 全为 DATA 引用，**无 call 不代表无 caller**。

## 1. 反射来源（观察）

`shader_pipeline::compiled::CompiledShaderPipeline` —— **外部依赖 crate，未逆向**（按项目规则不逆向通用库）。

观察到的字段布局：
| 偏移 | 内容 |
|---|---|
| `+0x40` | 描述符反射切片 |
| `+0x18` / `+0x20` | 顶点绑定（元素 0x38） |
| `+0xb8` | 顶点属性 |
| `+0xd0` | 计数 |
| `+0x148` | push constant 字节数 |
| `+0x58` | bit0 标志 |

**游戏自有侧只消费反射，不产生反射。**

## 2. 生产链（观察）

```
create_raster_pipeline  0x1416a3720  /  create_compute_pipeline  0x1416a4b20
  → create_descriptor_set_layouts      0x140be60a0
  → create_descriptor_set_layout 两特化
        0x1418c1a10（稀疏，HashMapIter）
        0x1418c2150（紧凑 slice，元素 40B）
      → vkCreateDescriptorSetLayout (设备分派 +0x230)
分配 set：
  DescriptorSetState::allocate_descriptor_set   0x140be5a40  （瞬时）
  create_persistent_descriptor_set              0x140bfef70  （持久，managed）
写 descriptor：
  update_persistent_descriptor_set              0x1416a2320
  bind_descriptor_set                           0x1416a5380 / 0x1416a5910 / 0x1416a6010
      → vkUpdateDescriptorSets (设备分派 +0x268)
bind：
  bind_persistent_descriptor_set                0x140bee150
      → vkCmdBindDescriptorSets (设备分派 +0x328)
```

**消费方**：`country_core::render::pass_builder::{draw_stream::render, compute_pass_builder::dispatch_impl}` —— 与 A2 的生产者清单相互印证。

## 3. `DescriptorSetState` 分配/复用原策略（观察，父级已核对关键点）

- `+0x58` 自旋锁，保护 `+0x60` 空闲池 `Vec`；
- `+0x50` 共享来源（原子引用计数 + 待用批次 `+0x20`/`+0x28`）；
- `+8`/`+0x10` 回退 pool + layout；
- 第二参为 Vulkan 分派：`+0x1c8` 分配 / `+0x1d8` reset / `+0x1e0` 释放 / `+0x5c8` vkDevice。

流程：取锁 → 池非空则弹末尾 reuse（reset）→ 空则从来源补批 → 源也空则新建一个 set。

**池耗尽重试（父级核对，`batch05-0x140be5a40.c:191`）**：

```c
if ((iVar12 != -0x3b9bd788) && (iVar12 != -0xc)) break;
```

`-0x3b9bd788` = `-1000069000` = **`VK_ERROR_OUT_OF_POOL_MEMORY`**；`-0xc` = **`VK_ERROR_FRAGMENTED_POOL`**。
→ 这两种错误时把条目推入调用方回收 `Vec`(`param_3`) 并**循环重试**。**确认**。

> **结论：原策略是「空闲池复用」，不是内容寻址。** 这与候选实现不同（见第 7 节）。

## 4. layout 规则（观察；tag 映射为推断）

- 每 binding **0x18 字节**：`[0]` binding 号 / `[4]` type / `[8]` count / `[0xc]` stage / `[0x10]` samplers。
- stage：raster `0x1f`、compute `0x20`；**set#0 用传入参数，其余置 `0x7fffffff`**。
- tag → type（**推断**）：`0` 存储缓冲 7 / `1` uniform 6 / `2` combined image sampler / `3` storage image / `4` sampled image / `5` immutable sampler；`_dyn` 后缀（尾 4 字节 == `0x5f64796e`）→ dynamic(+2)。
- **运行时计数 `min(max(P-32, 0), 8192)`**，其中 `P = *(*(device+0x658) + 0x680)`（property 名 **unknown**）。
- 运行时置 `PARTIALLY_BOUND`(4)；sampled image 置 15 且 pool `UPDATE_AFTER_BIND`(2)。
- immutable sampler：`FUN_140bfc5b0(device, name)` 取句柄 → 调用方 sampler 缓存（≤8 内联 + `0x58` 字节节点）。两个特化的 switch 一致。
- 持久 set：`maxSets=1`、`flags=2`、managed tag=`6`。

## 5. persistent set（观察）

- `create_persistent_descriptor_set`：`create_descriptor_set_layout`（set 切片步长 0x28，stage `0x7fffffff`，sampler 缓存）→ 按 type 累加 → `vkCreateDescriptorPool` (+0x240, `maxSets=1`, `flags=2`) → `vkAllocateDescriptorSets`（变量计数同第 4 节规则）→ `ManagedResourceStore::allocate` (`0x1418aead0`) → 结果 0x18 字节，tag=`6`。
- `get_vk_descriptor_set`：校验 `+0x16 == 6` 且 `+0x14 == *(store+0xa50)`。
- `update_persistent_descriptor_set`：构造 `VkWriteDescriptorSet`(0x40) → `vkUpdateDescriptorSets` (+0x268)，`device+0xab0` 原子 +1。
- `bind_persistent_descriptor_set`：`get_tracked` → tag==1 → `vkCmdBindDescriptorSets` (+0x328)。

## 6. 管线创建（观察）

- **raster**：leek(0x14) → `create_descriptor_set_layouts(param_3+0x40, 0x1f)` → push constant 仅当 `+0x148 != 0`（stage `0x1f`）→ `vkCreatePipelineLayout` (+0x210) → 顶点属性 `map_vertex_attribute_format` → `vkCreateGraphicsPipelines` (+0x1f8, cache=`device+0x808`) → `create_descriptor_set_states` → 返回 0x38 字节。
- **compute**：同上，但 stage `0x20`、`vkCreateComputePipelines` (+0x200)。
- **变体/阶段选择依据完全来自外部 `CompiledShaderPipeline`**；游戏侧**没有 shader→pass 的选择逻辑**（在 `pass_builder`）。

## 7. 候选实现不一致（重要）

- `src/raw/descriptor_set.rs` 的 `DescriptorSetState` 用**按写入内容寻址的缓存**（`BTreeMap<Vec<u64>, Arc<Set>>`）。原侧是**空闲池复用 + 回收重试**。**这是候选自己发明的机制，不是原作行为。**
- `rg/binding.rs` 的 `rg_binding_to_descriptor_binding` 槽位变体对应**未证实**（源函数 `0x14205eff0` 是跳表，Ghidra 跳表损坏）。
- 候选的管线 cache / push constant / descriptor-states 接线**未由 `CompiledShaderPipeline` 驱动**。

## 8. 观察 / 推断 / 未知

**观察**：设备分派偏移、空闲池分配策略与错误码重试、运行时计数规则、type 映射、persistent 参数、反射字段布局、管线创建调用序列。

**推断**：tag `2/3/4/5` 与 `_dyn` 分支语义、`0x14205eff0` 映射（跳表损坏，无法坐实）。

**未知**：
1. 无 disassembly（本库返回 0 指令）→ 跳表分支无法坐实。
2. `device+0x658 → +0x680` 的 property 名 unknown。
3. 外部 crate `CompiledShaderPipeline` 的反射**生产**未恢复（按边界不逆向）。
4. 真实 pass → shader → descriptor set 的选择未动态确认。
5. `FUN_1418b6140`、sampler 缓存 `FUN_140bfc5b0` 内部未展开。

## 9. 复现

```
node scripts/render-recovery/run-job-queued.mjs <job.json> <REPORT.json> --config D:/tools/reverse-engineering/verification/server-config.json
```
（`job2.json` 为补采；`0x140be67f0` 在本库 decompile/disassemble 均失败，其 blend/vertex 映射已由 `reconstruction/rhapsody-rebuild/evidence/shader-production-validation.json` 另行验证。）
