# N4 取证：rg 剩余模块函数体

- 目标：`workspace/input/tiny-glade.exe`，SHA-256 `f507a6cea25c5199447b88ba746a5293e64a279f73797ca74c1ce7f4ff2b7d03`。
- 工具：RE-MCP Ghidra + `run-lanes.mjs`。产物：`job-a-core.json`/`REPORT-a.json`、`job-b-api.json`/`REPORT-b.json`、`job-c-tiny.json`/`REPORT-c.json`，保留 **69 份**原侧 `0x*.c`。
- 仅取证，未修改任何游戏代码。

## 0. 父级复核

`populate_resources 0x140be7ba0` 的结构论断已逐行核对：元素 **stride 0x48**（`0x140be7ba0.c:333-345`）、**`+0x36` 是用途字符**、物理记录 **stride 0x40**（`:788`）、registry **`[0x19]`/`[0x1a]`** 为待处理 index 列表（`:290/:292`）。**确认。**

## 1. `populate_resources 0x140be7ba0`（补 A2 的缺口）

签名 `(registry, ResourceInfo 容器, ResourcePool, backend)`。

**registry qword 布局**：

| 槽 | 内容 |
|---|---|
| `[1]` | 每 index 一个槽的数组（**步长 4**：u16 物理句柄 + u8 标记） |
| `[2]` | 槽数 |
| `[4]` | 状态字节数组 |
| `[7]` | 物理记录数组（**步长 0x40**） |
| `[8]` | 其长度（**须 < 0x10000**） |
| `[0x19]` / `[0x1a]` | 待处理 index 列表（起/止） |

- `ResourceInfo` 容器元素 **步长 0x48**；函数末尾析构并清空。
- 0x48 描述符：`+0x00` = 来源种类（**0 创建 / 1 unowned / 2 owned**）、`+0x36` = 用途种类、`+0x38`/`+0x40` = 句柄。
- 主循环按 **来源 × 用途** 分支：
  - 种类 0 → `take_texture 0x140be6b90` 复用，否则 backend 创建；
  - 种类 2 → `take_buffer 0x140be71e0` + helper；
  - 成功则把物理索引（u16）写回 `registry[1] + i*4`，并把 0x40 记录追加到 `registry[7] + idx*0x40`。

⇒ **"每 index 一个 u16 句柄 → 0x40 物理记录"，优先池复用。**

xref 仅 1 条 DATA（函数指针表），无直接 CALL。

> ⚠️ **该函数 0x48 元素 vs A2 的 12 字节 PassBuilder 账本是不同结构**，勿混。

## 2. `resource_pool`（观察）

两张 SwissTable：

| 表 | ctrl | mask | len | 键 | 值 |
|---|---|---|---|---|---|
| 纹理 | `+0x00` | `+0x08` | `+0x18` | 纹理描述符（~0x1f 字节，逐字段全等） | `Vec<0x38 字节记录>` |
| 缓冲 | `+0x30` | `+0x38` | `+0x48` | — | `Vec<0x28 字节>` |

- `take` = **LIFO 弹栈**（`len @ entry-8--`，`base @ -0x10`）；未命中写标签 **7/5**（`populate` 据此走"需新建"）；
- `insert` = push 进该键的 `Vec`，新建桶。

⇒ **按描述符键的"每键独立空闲栈"，不是模糊匹配。** 这与候选的"精确匹配池"方向一致。

`clear 0x140be76f0`：排空缓冲表并对 raw 做原子引用递减 / 归还设备分配器。
（**存疑**：其 PDB `size = 27` 与约 160 行的函数体不符 —— 记录为可疑，不据此断言。）

## 3. `temporal`（观察）

`State` = `[0..0xa8)` 头部 + **表 A**（时态资源：ctrl`+0xb0` / mask`+0xb8` / len`+0xc8`；**桶 0x68**：`-0x60` 状态（0=raw/1=buffer/2=exported）、`-0x5c` 访问码、`-0x58`/`-0x48` 原始/导出句柄）+ **表 B**（parity：ctrl`+0xe0`，键 u64 / 值 0-1）。

- `get_and_flip_parity 0x140beced0` 翻转并返回；
- `export_temporal 0x1404f7ca0` 遍历表 A：状态 0 → `impl$0::export 0x140bd6380`、状态 1 → `impl$1::export 0x140bd5120`，**传访问码 `0x37`**（**与 A2 的"0x37 = 写 rg"互证**），并写状态 = 2；
- `get_or_create_temporal 0x14096c890`/`0x14096ce90`（`impl$11` → `import_unowned 0x140bd60a0`）、`0x14096c310`（`impl$12` → `0x140bd4fc0`）：**命中状态 2 则重挂（→ 1）**，未命中则新建挂表；
- `get_or_create_temporal_pair 0x14096d490` 用 parity 决定双缓冲 A/B 读写槽。

**状态转移**：`new(0) → export → 2 → 次帧 reimport → 1`。

## 4. `pass_api` / `recorder`（观察）

- `RenderPassApi` 是**设备 vtable 的类型化薄封装**：统一先校验（格式短字 `+0x14` 对 `device+0xa50`、`+0x16 == 1`、generation），再调 vtable（如 `copy_buffer` 调 `(**(param_1[1] + 0x370))`）。
- `begin_render_pass 0x1416f6e90`：对每个 attachment（**≤9，步长 0x60**）调 `ResourceRegistry::image_view 0x140be7a60`，再建 pass（vtable `+0x418`）。
- `recorder` 的 `impl$1::render` 是 **country_core 的具体回调**（如 `0x140904990` = registry.buffer + fill_buffer；`0x140904a30` = image_from_raw_handle + clear_color_image）。

`can_be_concurrent_with 0x140bece70` —— N4 概括为"**读-读并发、写冲突**"。

> ⚠️ **父级注记（与 A3 的表述差异）**：A3 给的是更细的规则（`classify(v) = (v-0x28) < 0x10`；两类都不在区间→类相同即并发；否则需类相同 **且 group != 0 且相等**）。N4 的概括**不与 A3 冲突，但更粗**。以 A3 的逐行版本为准；R2 正在做全量收口。

**未能取得**：小访问器 `0x140b25c90` / `0x140bece40` / `0x140bece50` / `0x140bece60` 等，decompile **与** disassemble 均返回 error envelope。

## 5. `import_export`（观察）

以 0x48 描述符的 `+0x00` 编码种类：
- `import_unowned`（`0x140bd60a0`/`0x140bd4fc0`）：对 lifetime tracker（`[1]+8`）**原子 +1**、种类 = 1；
- `import_owned`（`0x140bd6220`）：种类 = 2；
- `export`（`0x140bd6380`/`0x140bd5120`）：向导出向量追加 **0x30 字节** `{u32 tag, 4 句柄, u32, u8 访问码}`；
- `from_owned_rg_resource`（`0x140bd63f0`）：要求标签 0，否则 panic；
- `into_sampled_rg_texture`（`0x140bd6470`）：掩码 `0x2200`。

## 6. `dynamic_constants`（观察）

**覆盖 uniform/storage 的 bump 分配器，页 = 0x40000（256 KiB），usage 0x20033。**

- `allocate 0x140c02740`：对齐默认 0x10、`off = align_up(cur, align)`；越页则 `allocate_buffer 0x140c029f0` 新建 256 KiB 页并 `cur = 0` 重试；
  返回 `{device_addr, size, host_ptr, off, 页句柄, 引用计数, 格式 +0x34/+0x36}`。
- `new 0x140c02570`：建首页且偏移 = 0（即 reset 点）。
- binding 建造器（`0x140c022b0` 等）由 chunk 生成 `{tag 6, device_addr, size, offset, 页, 格式}` 描述符。

## 7. 仍未知

- 短访问器（`PassResourceAccessType::new`、`PassResourceRef::resource_index`、`ResourceSyncGroup::new`/`is_equivalent`、Ref 访问器、`end_render_pass 0x1416f7850`、`dynamic_constants` 的 `device_address`/`into_gpu_lifetime_tracker`）**函数体未取得**；
- access_type `0x17`/`0x19`/`0x1d`/`0x1e`/`0x24` **语义名未定**；
- xref 仅 DATA，caller 关系未闭合。

**字段名均为结构观察，非作者命名。**
