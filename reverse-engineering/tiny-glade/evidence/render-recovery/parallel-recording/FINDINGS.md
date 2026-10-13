# 原作并行录制：record_batch 分块算法与 PGO 统计

- 日期：2026-10-13。对象 `tiny-glade.rne`（SHA-256 `f507a6ce…7d03`），PDB `2d9686b8…8911`。
- 方法：PDB 骨架（内联/调用关系）+ 只读 PE 直读（`scripts/render-recovery/pe_readonly.py`）。纯静态。
- 置信度：**H** 指令直读或 PDB 符号直接给出；**M** 由形状推断；**L** 猜测。
- **更正**：`producer-closure/SYNTHESIS.md:106` 的"无并行录制证据（单活跃 CB）"不成立，应为"按块并行录制，每块一个活跃 CB"。

## 1. 结构（H，PDB）

`CompiledRenderGraph::record_batch<PassResources>`（RVA `0x2c1d40`，源码行 130）：

| 闭包 | 源码行 | 内联/调用 | 角色 |
|---|---|---|---|
| 本体 | 130 | `ComputeTaskPool::get`、`par_chunk_map_mut`、`BufferBarrier::new`、`ImageBarrier::new`、`PassPgoStats::duration_ns`、`CommandBuffer::end` | 串行段：构造屏障、分块、派发、合并 |
| closure$1 | 162 | `issue_and_reset_pipeline_barrier`、`handle::track`、`RuntimeResource::borrow`、`leek::begin/end_rg_pass`、`Instant::now/elapsed`、`PassPgoDuration::new`、puffin 作用域 | **每个 pass**：发屏障、跟踪资源、计时、调回调 |
| closure$2 | 300 | 内联 closure$1；块内 `CommandBuffer::begin` | **每块**：返回 `(CommandBuffer, Vec<PassPgoDuration>)` |
| closure$3 | 315 | `PassPgoStats::extend` | 合并各块耗时 |

`par_chunk_map_mut` 的元素类型是 `Vec<&mut [RecordedPass<PassResources>]>`：先切成**变长连续块**，再让任务池逐块执行（H）。
线程局部存储只出现在 puffin 分析器，**不是**每线程命令池（M）。

## 2. 分块算法（H，指令直读）

```
stats   = 上一帧的 PassPgoStats            // 见第 3 节
total   = stats+0x30, max = stats+0x38
budget  = clamp(max, total/16, total/2)    // 0x1402c21fb..0x1402c2220：shr 4 / shr 1，两次 cmov；total/16 > total/2 时 panic
chunks  = []
acc = 0
for pass in batch.passes:                  // RecordedPass 步长 0x40；名字在 +0x18/+0x20
    acc += stats.duration_ns(name) or 0    // 0x140bea1e0：hashbrown 查表，缺失记 0
    if acc >= budget: 关闭当前块，acc = 0  // 0x1402c229b: cmp r15,rsi; jae
chunks: Vec<(ptr,len)>，16 字节元素         // 0x1402c224f
if 并行关闭: 串行路径 0x1402c3240
if len(chunks) <= 1: 串行                  // 0x1402c2493
else: ComputeTaskPool::get → par_chunk_map_mut
```

- 并行开关（M）：全局字节 `0x14362c228` 为 2 时改用 `device+0xabb` 取反；字段名未知。
- 合并后把每个 `PassPgoDuration` 写入**本帧**统计：`total += d`，`max = max(max, d)`（`0x1402c23c2..0x1402c23d1`，H）。

## 3. PGO 统计是双缓冲（H）

`GraphExecutionRecorder::end_frame`（`0x140bec400`）内联 `PassPgoStats::reset`：

- `0x140bec8df..0x140bec933`：交换两份 0x40 字节的统计（`+0x48` 与 `+0x88`）。
- `0x140bec937..0x140bec99c`：把交换后的"本帧"清空：hashbrown 控制字节填 `0xff`、`items=0`、重算 `growth_left`、`total=max=0`。

推出布局（M）：`PassPgoStats = { hashbrown RawTable + hasher (0x30), total_ns: u64 @0x30, max_ns: u64 @0x38 }`，键为 pass 名。

⇒ 分块依据的是**严格上一帧**的耗时，每帧重建，不会跨帧累积。

## 4. 推出的边界行为（M，由第 2、3 节推得，未动态验证）

- **第一帧/统计为空**：`total=max=0 ⇒ budget=0`；条件 `acc >= 0` 恒真 ⇒ **每个 pass 各自成一块**。
- **新出现的 pass**：耗时记 0，会被并入下一个有耗时的 pass 所在块。
- 块数下限约为 2（只要存在 `max < total/2`），上限约为 16（预算不小于 total/16）。单个特别重的 pass 独占一块。

## 5. 仍未知

- 每块命令缓冲的来源（Vulkan 命令池需外部同步）。字节里未见线程局部的池。
- `device+0xabb` 开关的含义与默认值。
- 块结果合并成提交时，命令缓冲的数组顺序（推断为块顺序，M）。
