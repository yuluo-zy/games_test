# A3 取证：barrier / recorder 原规则

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra（`run_auto_analysis:false`）+ `run-job-queued.mjs`。job/报告见 `job*.json` / `REPORT*.json` / `probe-REPORT.json`；保留伪代码：`0x1402c1d40.c`(record_batch)、`0x1402c3240.c`(逐 pass 录制器)、`0x1402c3d00.c`(add_pre_pass_barriers)、`0x140bd66a0.c`、`0x140bd6810.c`。
- 仅取证，未修改任何 Rust 源码。

## 交付边界

Ghidra 伪代码不是作者原 Rust 源码；PDB 签名普遍 `void()` 占位。`run_auto_analysis:false` 下 `get_xrefs_to` 只回 DATA/重定位（vtable 槽），**空 xref ≠ 无 caller**；本报告的调用链靠伪代码直接观察得到。无动态证据。

## 1. 同步组语义（观察）

| 地址 | 函数 | 观察到的行为 |
|---|---|---|
| `0x140bece50` | `ResourceSyncGroup::new` | 全局 `AtomicU32`（`0x14362c230`）`fetch_add(1)`；该全局 `.data` 初值 = **2** → 组 id 从 **2** 起、非零、单调递增 |
| `0x140bece60` | `ResourceSyncGroup::is_equivalent` | 两个 u32 相等 |
| `0x140b25c90` | `PassResourceAccessType::new` | `mov %edx,%eax; mov %ecx,%edx` —— **双字段调换**（说明该类型是两个 u32 字段的交换构造） |
| `0x140bece40` | `PassResourceRef::resource_index` | `mov (%rcx),%eax` —— 记录 `+0` |

**推论**：同步组 id 非零且全局单调 —— 这正是 `can_be_concurrent_with` 里"要求 `group != 0`"能够成立的前提，与第 2 节互证。

## 2. `can_be_concurrent_with` 与分类器（观察）

`0x140bece70` `PassResourceAccessType::can_be_concurrent_with`：

```
classify(v) = (v - 0x28) < 0x10          // v ∈ [0x28, 0x37]
若 两者类都不在 [0x28,0x37]  → 类相同即可并发
否则                          → 需 类相同 且 group != 0 且 group 相等
```

**外部依赖边界**：`FUN_140c10e10`（来自 `vk-sync-fork`）指令为
`add $-0x28,%cl; cmp $0x10,%cl; setb %al`
→ 入参是 access_class **单字节**，返回 `class ∈ [0x28, 0x37]` 的布尔。**按项目规则只登记边界，不逆向重写该库。**

**未知**：访问类别枚举**未命名**（[0x28,0x37] 区间里每个值的语义未知）。

> **父级交叉核对（未决）**：A2 在 producer 调用点实测到的访问码是 `0x1a`(compute 读) / `0x24`(draw 读) / `0x37`(写 rg) / `0x02`(间接绘制派发)。其中**只有 `0x37` 落在本节的 `[0x28,0x37]` 区间内**。这意味着要么还有未观察到的 `[0x28,0x37]` 访问码，要么账本里的 access_type 在进入 compiled 记录前还经过一次变换。**该对应关系尚未闭合**，不要在后续工作中把 A2 的访问码直接当成 A3 的 class。

## 3. 屏障生产顺序（观察，`record_batch` `0x1402c1d40`）

三阶段：

1. **initial**（每 batch 一次）：drain `this+0x210` 列表 → 逐资源用 lifetime 首访问（`this+0x128`，元素 0x18 字节）与资源 initial 码构造屏障 → 压入 scope(`this+0x1b0`) → `is_empty` → `issue_pipeline_barrier` → `reset`。
2. **逐 pass**：调 `FUN_1402c3240`。其 per-pass 序列为
   `new scope → add_pre_pass_barriers(0x1402c3d00) → issue_pipeline_barrier → reset →（若 device+0xab8）issue_full_pipeline_barrier → begin → pass 回调 → end`。
3. **export**：仅 **final batch**（标志 `puStack_1a0[2] != 0`），遍历 export 表（`this+0x20`，元素 0x30），`from = last 访问码`（否则 initial），`to = 记录+0x28` → 压 scope → issue → reset。

## 4. `add_pre_pass_barriers` 核心规则（观察；父级已逐行核对 `0x1402c3d00.c`）

访问记录 **stride `0x14`（20 字节）**：

| 偏移 | 含义 |
|---|---|
| `+0x00` | 资源句柄（喂给 `FUN_140be78b0` 解析） |
| `+0x04` | **上一访问索引**；`0xffffffff` = 无历史 |
| `+0x0c` | sync group |
| `+0x10` | access class |

判定（**父级逐行确认**）：

- **无历史**（`+0x04 == 0xffffffff`）且"需 initial 屏障"标志置位 → 生成 `from = 0`（initial）→ `to = 本次 class` 的屏障。
- **有历史** → `FUN_140bece70(&cur, &prev)` = `can_be_concurrent_with(cur, prev)`；**仅当返回 false（不并发）**才生成 `from = prev.class` → `to = cur.class` 的屏障。
- 资源 kind（`FUN_140bd6c80` 返回 0/1）决定屏障类型：
  - kind 0 → `ImageBarrier::new`(`0x140bd65e0`) + `image_barrier`(`0x140bd66a0`)
  - kind 1 → `BufferBarrier::new`(`0x140bd6610`) + `buffer_barrier`(`0x140bd6810`)

即：**原作者不是"每两个 pass 之间无条件插屏障"**，而是按具体资源的前后访问类别与同步组判定。

## 5. `raw::barrier` 结构（观察）

| 地址 | 函数 | 观察到的行为 |
|---|---|---|
| `0x140bd65e0` | `ImageBarrier::new` | 填 `{+0 image, +8 aspect, +0xc = -1 (8B), +0x14 from, +0x15 to, +0x16 = 0}` |
| `0x140bd6610` | `BufferBarrier::new` | 填 `{+0 buffer, +8 = -1 (8B), +0x10 from, +0x11 to}` |
| `0x140bd6630` | `image_aspect_mask_from_format` | `fmt-0x7c > 6` → 1；否则查 **7 项表**（仅 depth/stencil 格式非 1） |
| `0x140bd6650` | `SynchronizationScope1::new` | 清 3 个向量（长度在 `+0x10`/`+0x28`/`+0x40`）；掩码 `+0x48 = 0x0000200000000001` |
| `0x140bd66a0` / `0x140bd6810` | `image_barrier` / `buffer_barrier` | 经 `FUN_140c10af0`/`FUN_140c10940` 把 layout → 访问掩码、OR 进 `+0x48`、分别向 **0x48B / 0x38B 元素**向量追加 |
| `0x140bd6950` | `reset` | 清三个长度 |
| `0x140bd6980` | `is_empty` | 三者皆 0 |
| `0x140bee030` | `issue_pipeline_barrier` | 传三数组 + 两个 stage u32 → `callq *0x3e0(%device)` |
| `0x140bee090` | `issue_full_pipeline_barrier` | 造单条全屏障（`src = dstStage = 0x10000`），同样走 vtable `+0x3e0` |

`SynchronizationScope1` 持有**三个向量**（元素 0x18 / 0x38 / 0x48 字节）+ 累计掩码 + `reset`/`is_empty`。

## 6. 观察 / 推断 / 未知

**观察**：第 1 节同步组、第 2 节并发判定与分类器指令、第 3 节三阶段顺序、第 4 节记录字段与判定规则、第 5 节结构字段与 issue 路径。

**推断**：三类记录（元素 0x18 / 0x38 / 0x48）对应 `vkCmdPipelineBarrier` 的 memory / buffer / image 数组；vtable `+0x3e0` = `vkCmdPipelineBarrier` 绑定；`PassResourceRef` / `AccessRecord` 的字段命名。

**未知**：
- 访问类别枚举未命名。
- 全局开关 `0x14362c220`（`==1`）/ `0x14362c228`（`!=2`）语义未知。
- `FUN_140bd6a70` 语义未知。
- 记录 → Vulkan 结构的完整字段映射 unknown。
- 无动态证据。

## 7. 候选实现差异

1. **`Access.write` 位不是原机制**：原侧"是否需要同步组"来自**外部 classify 的区间 `[0x28,0x37]`**；候选 `barrier.rs` 用 write 位近似 → **候选规则**。
2. **候选逐 access 直接展开 transition**；原作在 `add_pre_pass_barriers` 用 `+0x04` 上一索引 + `can_be_concurrent_with`，**仅不并发才生成**。
3. 候选 `compiled.rs` **没有** `SynchronizationScope`（三向量 + 累计掩码 + `reset`/`is_empty`）、**没有** per-pass `issue_full_pipeline_barrier`、**没有**每 access 打点 / begin-end / profiler。
4. 候选 `record_batch` 把三个 scope 平铺；原作**逐 pass 下沉到 `FUN_1402c3240`**，initial / export 在内联块。
