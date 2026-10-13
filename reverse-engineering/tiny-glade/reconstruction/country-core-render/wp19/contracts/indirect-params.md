# WP19 契约 4/4 — indirect_params / render_loop::resources::critters

版本 **B**。模块：`systems/render_loop/indirect_params.rs`（module 1619）、
`systems/render_loop/resources/critters.rs`（module 525）。数据源：PDB 骨架
structures.json.gz + procedure_occurrences（消费方类型单态化）。

标注：**O**=PDB/反编译；**I**=推断；**U**=未恢复。

## 1. indirect_params（0 独立函数，6 个命名类型）

本模块**没有独立过程**（O：PDB functions=0）。全部为间接绘制参数缓冲的 **类型标记
（type marker / SSBO 载体）**，经 `AssetSsboLibrary::add_new<T>()`/`get_rg_buffer<T>()` 与
`pass_resource_access::{read,write}_access_buffer<T>()`/`ComputePass::{read,write}_storage_buffer<T>()`
单态化引用（O，procedure_occurrences）。

| 类型 | O 消费方（过程名） |
|---|---|
| `AtomicCounterBuffer` | `pass_resource_access::read_access_buffer`、`write_access_buffer`、`AssetSsboLibrary::add_new`、`get_rg_buffer`、`ComputePass::write_storage_buffer`、`TypedStorageBuffer<AtomicCounterBuffer>` |
| `DispatchIndirectBuffer` | `add_new`、`get_rg_buffer`、`TypedStorageBuffer<DispatchIndirectBuffer>`、`render::ssbo::new_storage_buffer` |
| `DrawIndexedIndirectBuffer` | `read/write_access_buffer`、`write_access_buffer_concurrent`、`add_new`、`get_rg_buffer`、`ComputePass::write_storage_buffer(_concurrent)`、`read_storage_buffer` |
| `DrawIndirectBuffer` | `write_access_buffer`、`add_new`、`get_rg_buffer`、`ComputePass::write_storage_buffer`、`TypedStorageBuffer` |
| `DrawIndexedIndirectBucket` | 无过程引用（纯元素类型标记） |
| `DrawIndirectBucket` | 无过程引用（纯元素类型标记） |

映射（I）：`DrawIndexedIndirectBuffer`↔`render::indirect.rs::DrawIndexedIndirectBuffer`
（VkDrawIndexedIndirectCommand 缓冲，20B/条）；`DrawIndirectBuffer`↔
`DrawIndirectCommand`（16B/条）；`DispatchIndirectBuffer`↔`DispatchIndirectCommand`
（12B/条）；`AtomicCounterBuffer`↔渲染原子计数缓冲；`*Bucket` 为分桶的间接命令子数组
（nani GPU-driven 分桶，INDEX.md §3/W22/W28）。尺寸与元素数 U。

## 2. render_loop::resources::critters（0 独立函数，4 个命名类型）

同样为**类型标记**：小动物渲染 SSBO 载体（O，无 functions）。消费方（O）：
`country_core::render::pass_builder::…` 单态化（`add_new`/`get_rg_buffer`/access 族，
记录于 structures procedure_occurrences，module 518 区域）。

| 类型 | 语义（I） | 证据 |
|---|---|---|
| `FireflySsbo` | 萤火虫实例数据（位置/相位），供 `_critters_firefly` shader | INDEX.md §23 critters 族（1C+10R）；nani 小动物系统 |
| `ButterflySsbo` | 蝴蝶实例数据 | 同上 |
| `FireflyVisibilitySsbo` | 萤火虫可见性查询结果（`_critters_firefly_vis_query` compute 写） | INDEX.md §23 |
| `SheepAnimationSsbo` | 绵羊动画数据 | 同上 |

field 布局全 U（typed gap）；`nani_update_*` 小动物系统通过 `pass_resource_access` 读写
这些 SSBO（I；消费在 WP6 nani 系统侧）。

## 3. 通用 typed gaps
- 各类型 `size_bytes`/`alignment`/`fields`：PDB 全 unknown。
- 与 `resources::critters`（WP6，Wren 组件）区分：本模块是 **render_loop 域 GPU SSBO
  载体**，`resources/critters.rs` 是 CPU 组件/参数。
