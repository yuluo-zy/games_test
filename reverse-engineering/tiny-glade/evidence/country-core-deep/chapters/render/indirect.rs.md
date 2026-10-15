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

## 实现方式（算法级）（2026-10-15 render 域算法级深挖）

- **结构体语义**（O 步长证据）：`DrawIndexedIndirectCommand` 20B（`draw_indexed_indirect_raw` 步长 0x14）、`DrawIndirectCommand` 16B、`DispatchIndirectCommand` 12B —— 与 Vulkan ABI 对齐（I）。
- **消费链**：
  1. `reset_indirect_args` 的 0x1e0B 暂存区 = **24 × 20B `DrawIndexedIndirectCommand`，仅 `index_count` 非零**（round3 O：memcpy 静态表 0x142aaaa78，`[0x1b,0x12c,0x60,0x39,0x24,0x6,0x3c,0x6,0x30,0xae,0,0,0x10b,0x126c,0x42,0xb4,0xf,0x384,0x12,0x384,0x12,0x12,0x12,0x12]`）——初始化间接绘制命令；
  2. `indirect_params` 四个真 SSBO（AtomicCounter 8B / DrawIndirect 0x10 / DispatchIndirect 0x80 / DrawIndexedIndirect 0x1e0，SsboId=xxh3(类型名)）经 AssetSsboLibrary 注册；
  3. GPU 侧：nani/grass 的 `draw_indexed_indirect_count/raw` 消费这些命令（`DrawBuilder` 种类 7/8）。
- 字段顺序不升级为 O 的原因：需对照着色器侧 `StructuredBuffer` 消费（WP1 nani cull）后才能定。

## 纠错/差异

- 字段顺序按 Vulkan ABI（I）；若 WP1 以字节传 `ash::vk` 结构，字节序须与 `Vk*IndirectCommand`（20/16/12 B）一致——在对照着色器侧 `StructuredBuffer` 消费（WP1 nani cull）前，字段顺序不升级为 O。
- 消费者：CPU 侧命令构建器 → 间接 GPU 缓冲（`indirect_params` 的 `DrawIndexedIndirectBuffer` 等，经 AssetSsboLibrary 注册）；`reset_indirect_args` 的 0x1e0 B 暂存区 = 24 条仅 `index_count` 非零的 `DrawIndexedIndirectCommand`（render-gap §3.2 O）。
