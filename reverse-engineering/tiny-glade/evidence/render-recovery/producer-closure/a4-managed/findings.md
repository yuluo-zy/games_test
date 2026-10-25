# A4 取证：managed 生命周期原 ABI

- 目标：原发行二进制 `D:/game/reverse-engineering/tiny-glade/workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra（`run_auto_analysis:false`）+ `run-job-queued.mjs`。产物：`job.json`、`REPORT.json`、26 份 `0x*.c`、8 份 `xrefs_*.json`。
- 仅取证，未修改任何 Rust 源码。

## 交付边界

Ghidra 伪代码不是作者原 Rust 源码；PDB 签名普遍 `void()` 占位。`run_auto_analysis:false` 下 xref 仅 1 条 DATA 引用（都在 `0x1436A4*`/`0x14372*` vtable 区）→ 经函数指针**间接调用**，**空 CALL 边 ≠ 无 caller**。下文区分**观察**与**推断**。

## 1. 句柄 ABI：`RawManagedHandle` = 0x18 字节（观察，父级逐行核对）

| 偏移 | 大小 | 含义 |
|---|---|---|
| `+0x00` | u64 | 指向所属 **store**（drop 里当 store 基址用） |
| `+0x08` | u64 | 指向 **inner 引用计数块** |
| `+0x10` | u32 | **generation** |
| `+0x14` | u16 | **store id** |
| `+0x16` | u8 | **资源类型 tag** |

**inner 块**（`*(handle+8)`）：

| 偏移 | 含义 |
|---|---|
| `+0x00` | atomic u64 = **tracked 集合 / 命令序列高水位**（`track` 与 `get_tracked` 都做 `atomic_max`） |
| `+0x08` | atomic u32 **CPU 强引用计数** |
| `+0x0c` | u32 generation |
| `+0x10` | 资源值 `T` |

## 2. `ManagedResources` ≈ 0xa52 字节（观察）

- `+0x000` ptr → 待回收 `Vec`（0x20 字节，实为 `Mutex<Vec<16B 记录>>`）；
- `+0x008` 起 **7 个同构 store，每个 0x178 字节**；
- `+0xa50` u16 store id；
- free-list 头在**每个 store 基址 +0x170**。

**store 顺序**（由 `dispose<T>` 实参 + free-list 偏移反推）：
`texture, buffer, swapchain, render pass, sampler, desc set, framebuffer`，
类型 tag 依次 **0, 1, 2, 4, 5, 6, 7**（注意**跳号：无 tag 3**）。

## 3. `drop` 与退役条件（观察；父级逐行核对 `0x140be24b0.c`）

`managed::handle::impl$6::drop`（`0x140be24b0`）：

1. `inner = handle+0x08`；若为 0 → panic（`UNK_142b12c50`）。
2. `LOCK; *[inner+8] -= 1; UNLOCK` —— 原子递减 CPU 引用计数。
3. 若 `*[inner+8] != 0` → 直接跳末尾（`handle+8 = 0`，return）。
4. **递减到 0 时**：`store = *[handle+0x00]`，对 store 首字节做自旋 try-lock，然后
5. 从 handle 读出 `inner`(+0x08)、`gen`(+0x10, u32)、`store_id`(+0x14, u16)、`tag`(+0x16, u8)，
6. 写入 store 的待回收 `Vec`（`ptr = store+0x10`，`count = store+0x18`），**记录 stride 0x10**：

   ```
   +0x00 u64 = inner
   +0x08 u32 = generation
   +0x0c u16 = store_id
   +0x0e u8  = tag
   ```

   → **16 字节退役记录 `{inner, gen, store_id, tag}` 确认**（父级核对，与 A4 报告完全一致）。
7. `*[store+0x18] = count + 1`（push），解锁，`handle+8 = 0`。

## 4. generation 规则（观察 + 推断）

- 强引用计数在 `inner+8`，**初始 1**（`allocate` 写入 1）。
- **仅在 `dispose<T>` 处理退役记录且 `inner+8 == 0` 时**对 `inner+0xc` **自增 +1**，随后把 inner 推入 free-list 复用。
- `allocate` 只重置 `inner+0x00 = 0`、`inner+0x08 = 1`，**不复位 generation**。
- 旧句柄因 `handle+0x10 != inner+0xc` 被拒：`track` 静默返回；`get_tracked` panic；`get_maybe_tracked` 返 `None`。

## 5. "CPU 退役可早于 GPU 完成"的机制（观察；父级核对 `0x1418afb90.c`）

**`ManagedBuffer::dispose`（`0x1418afb90`）不调用任何 `vkDestroy*`。** 父级逐行确认：

1. `[param_1+0x6c] = 2`（状态标记，防重复 dispose）；
2. 读出 managed buffer 的字段（`+0x20`、`+0x28..+0x60` 八个 u64、`+0x68` u32、`+0x6d` u16、`+0x6f` u8）；
3. **把裸句柄 `[param_1+0x20]`（VkBuffer）推入 `param_2+0x30/0x38/0x40` 的句柄表 `Vec`**；
4. **把 0x48 字节记录推入 `param_2+0xd8/0xe0/0xe8` 的记录表 `Vec`**；
5. 返回。**没有 vkDestroy。**

`ManagedTexture::dispose`（`0x140bdbe40`）同为延迟入桶。

`take_out_dropped_waste`（`0x1418c0a70`, 2207B）：取出待回收 `Vec` → 按记录 tag 分发到各 `dispose<T>` → 尾调 `FUN_140bf92f0(param_2)` 透传销毁桶。

> 与 A3 一致：销毁桶结构 = **句柄表 `Vec` + 0x48 字节记录表 `Vec`**。两个 agent 独立得到同一结构。

## 6. 类型擦除（观察）

- `get_tracked<Buffer/DescSet/RenderPass>` 断言 tag == `1`/`6`/`4`；
- 7 个 `dispose<T>` 断言 tag == `0/1/2/4/5/6/7`；
- 统一 `RawManagedHandle` 由 `0x141620a10` 的 mangled 名证实。

## 7. 关键地址

| 函数 | 地址 |
|---|---|
| `handle::track` | `0x140bd6a70` |
| `handle::clone` | `0x140bd6b00`（**本库无函数体**，但被 `0x141620a10` 当 call 目标调用） |
| `handle::eq` | `0x140bd6b30`（本库无函数体） |
| `handle::drop` | `0x140be24b0` |
| `ManagedResourceStore<T>::allocate` | `0x1418ae7e0` / `0x1418ae8a0` / `0x1418ae9f0` / `0x1418aead0` |
| 低层 allocate | `0x140bdbd50` |
| `get_maybe_tracked` | `0x1418aeba0` |
| `get_tracked<...>` | `0x1418aed30` / `0x1418aee50` / `0x1418aef70` |
| `ManagedResources::new` | `0x1418c08e0` |
| `take_out_dropped_waste` | `0x1418c0a70` |
| `dispose<T>` ×7 | `0x1418af090` / `220` / `390` / `500` / `690` / `800` / `fa20` |
| `ManagedBuffer::dispose` | `0x1418afb90` |
| `ManagedTexture::{dispose,new,view,create_vk_image_view}` | `0x140bdbe40` / `0x140bdc2d0` / `0x140bdc5d0` / `0x140bdcba0` |

## 8. 观察 / 推断 / 未知

**观察**：第 1、2、3、5、6 节的全部偏移、常量与控制流；第 4 节的 generation 递增点。

**推断**：
1. "tracked 高水位"假设序列为**全域单调序**（未观测到 domain 字段）；
2. store 内 `0x160` 容量 + 44 桶结构属分桶 slab/HashMap（**算法未定**）；
3. `clone`(`0x140bd6b00`) 语义为 refcount++ —— 仅按调用点推断（本库无函数体）。

**未知**：
- `FUN_140bf92f0` / `FUN_140bdfcf0` 内部如何用 tracked 高水位对 **completed timeline** 判定可释放；
- 销毁桶 `param_2` 的 0x48 字节记录字段名；
- 记录 tag 枚举中的 `case 9` 与**跳号 tag 3**；
- `clone` / `eq` 的逐字节语义。

## 9. 候选实现差异（`reconstruction/rhapsody-rebuild/src/raw/managed/`）

| # | 候选 | 原作 |
|---|---|---|
| 1 | 跟踪集合 `BTreeMap<domain, seq>` ❌ | **单个 atomic u64 高水位** |
| 2 | 句柄 `ResourceKey{store, index, generation}` ❌ | **类型擦除 tag + store_id + inner 自带 refcount/gen** |
| 3 | 7 个独立 Manager ❌ | **共享 store + 共享待回收 Vec** |
| 4 | 直接 `drop(Arc)` ❌ | **延迟入销毁桶**（不调 vkDestroy） |
| 5 | generation 递增在 `take_out_dropped_waste`(slot) ❌ | 递增在 **`dispose<T>`（inner）** |

方向是对的（CPU/GPU 退役分离、generation 防陈旧、free-list 复用），但**缺类型擦除与共享 store**，且 #4/#5 属机制性偏差。
