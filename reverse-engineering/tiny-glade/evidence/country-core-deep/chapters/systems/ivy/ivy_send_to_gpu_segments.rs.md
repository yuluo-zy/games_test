# systems/ivy/ivy_send_to_gpu_segments.rs

> 文件作用：**段 GPU 上传（SSBO）：收集全部 Ivy 段 → IvySegmentSsbo 缓冲上传。**
>
> 生产者→GPU 验收链**末段**（consumer）：grower 生长 → 本系统上传段 GPU。

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments` | 0x140a9b4e0 | 33 | 系统:上传 | O（2026-10-15 定点深挖）：全量段收集（IvyStorage::total_segment_count 预算）→ IvySegmentSsbo 缓冲重分配/上传 | collision-eco\decomp\decomp-0x140a9b4e0.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级，2026-10-15 O）

**ivy_send_to_gpu_segments（0x140a9b4e0，~10KB，Bevy 系统，puffin scope 串 `crates/country-core/src/systems/ivy/ivy_send_to_gpu_segments.rs` @0x142adb498）**

1. **puffin scope**：`func_0x000140c16a00()`；串 `country_core::systems::ivy::ivy_send_to_gpu_segments::ivy_send_to_gpu_segments::f`（@0x142adab98，0x4e 字符）。
2. **收集段数**：`FUN_141309c30(&Vec, &err, ...)` = `IvyStorage::total_segment_count` 的 `Iterator::collect`（fat ptr @0x142adab10 → `/rustc/.../core/src/iter/traits/iterator.rs`）。`lStack_90` = 段计数，`uStack_88` = 数据 ptr。
3. **SSBO 库查找（AssetSsboLibrary）**：`FUN_140969720(*param_2, &UNK_142c2e330)` —— **IvySegmentSsbo**（SsboId = **0x2002d2fbdff3d80f**）。缓冲类型名匹配：`memcmp(名称, &UNK_142a9e6a4, 0x44)`（`country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo` 全名）。
4. **缓冲重分配**：`uVar15 = 段数(lStack_80) × 0x30`；`plVar13[4] < 需` → 0xb 错误路径；否则 `FUN_140beefa0(&err, *(qword*)*param_3 + 0x10, buf+1, old_size, need, &staging, 1, 0)`（ssbo_library.rs 缓冲写入原语）→ 成功则逐段打包（栈上 uStack_168..d8 区 = 48B 打包元素）。
5. **失败路径**：`FUN_1428d9760("called Result::unwrap() on an Err value" @0x142adab48, 0x2b, ..., &UNK_142adb498)` = panic unwrap。

**关键事实（O）**：
- **GPU 段元素尺寸 = 0x30 = 48B**（`count * 0x30` 分配）。
- **IvySegmentSsbo 键 = 0x2002d2fbdff3d80f**（rne-read qword @0x142c2e330）。
- SSBO 名称全名（O）：`country_core::systems::ivy::ivy_send_to_gpu_segments::IvySegmentSsbo`（0x142a9e6a4 名表区）。
- 段收集 = `IvyStorage::total_segment_count`（chapters/resources/ivy/ivy_storage.rs 内联，全株段数求和作 GPU 上传预算）。

**跨模块**：消费 `IvyStorage` 全部株段（grower 产物）；输出 IvySegmentSsbo；render 域 `instanced_ivy` draw shaders（draw_instanced_ivy_branch_shader 0x1408dbc50 / ivy_leaf_shader 0x1408dc1c0）为 GPU 消费端。
