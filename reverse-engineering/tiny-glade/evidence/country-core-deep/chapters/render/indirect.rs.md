# render/indirect.rs

> 文件作用：**间接绘制/分派命令包装**——`DrawIndexedIndirectCommand`（20 B = `VkDrawIndexedIndirectCommand`）、
> `DrawIndirectCommand`（16 B = `VkDrawIndirectCommand`）、`DispatchIndirectCommand`（12 B = `VkDispatchIndirectCommand`）
> 与空命令/字段写入访问器。0 独立业务过程（全内联）。证据：
> `reconstruction/country-core-render/wp2/contracts/indirect.md`（O/I）。

函数总计 5：业务 0 / 内联无独立符号 5 / 派生胶水 0

## 布局表（I，尺寸来自 Vulkan ABI + DrawBuilder stride 证据）

| 结构 | 布局 | 证据 |
|---|---|---|
| `DrawIndexedIndirectCommand` | 20 B：`{ index_count: u32, instance_count: u32, first_index: u32, vertex_offset: i32, first_instance: u32 }` | `DrawBuilder::draw_indexed_indirect_raw` 步长 0x14 = 20 B（WP1 O） |
| `DrawIndirectCommand` | 16 B：`{ vertex_count: u32, instance_count: u32, first_vertex: u32, first_instance: u32 }` | Vulkan ABI（I） |
| `DispatchIndirectCommand` | 12 B：`{ x: u32, y: u32, z: u32 }` | Vulkan ABI（I） |

## 内联/无独立符号函数（全部 I，无机器码）

- `DrawIndexedIndirectCommand::empty`（行 13）— **I**：零初始化命令
- `DrawIndexedIndirectCommand::w_vertex_count`（行 17）— **I**：写 `vertex_count` 字段访问器（语义 U：内联）
- `DrawIndirectCommand::empty`（行 38）— **I**：零初始化
- `DrawIndirectCommand::w_instance_count`（行 42）— **I**：写 `instance_count` 访问器
- `DispatchIndirectCommand::empty`（行 62）— **I**：零初始化

## 派生胶水（0 项）

## 纠错/差异

- 字段顺序按 Vulkan ABI（I）；若 WP1 以字节传 `ash::vk` 结构，字节序须与 `Vk*IndirectCommand`（20/16/12 B）一致——在对照着色器侧 `StructuredBuffer` 消费（WP1 nani cull）前，字段顺序不升级为 O。
- 消费者：CPU 侧命令构建器 → 间接 GPU 缓冲（`indirect_params` 的 `DrawIndexedIndirectBuffer` 等，经 AssetSsboLibrary 注册）；`reset_indirect_args` 的 0x1e0 B 暂存区 = 24 条仅 `index_count` 非零的 `DrawIndexedIndirectCommand`（render-gap §3.2 O）。
