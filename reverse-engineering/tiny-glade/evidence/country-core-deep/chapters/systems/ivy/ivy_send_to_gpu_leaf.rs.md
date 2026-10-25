# systems/ivy/ivy_send_to_gpu_leaf.rs

> 文件作用：**叶/花 GPU 上传（SSBO）：收集全部 IvyLeaf/IvyFlower 实例 → IvyLeafSsbo + IvyFlowerSsbo 缓冲上传。**
>
> 生产者→GPU 验收链**末段**（consumer）：leaf_spawner 放叶 → 本系统上传 GPU。

函数总计 4：业务 1 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf` | 0x1408c1580 | 13 | 系统:上传 | O（2026-10-15 定点深挖）：全量叶/花实例收集 → IvyLeafSsbo + IvyFlowerSsbo 缓冲重分配/上传 | collision-eco\decomp\decomp-0x1408c1580.txt |

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 3 项

## 实现方式（算法级，2026-10-15 O）

**ivy_send_to_gpu_leaf（0x1408c1580，~15KB，Bevy 系统，puffin scope 串 `crates/country-core/src/systems/ivy/ivy_send_to_gpu_leaf.rs` @0x142aa1398）**

1. **puffin scope**：`func_0x000140c16a00()` 检查 profiler；串 `country_core::systems::ivy::ivy_send_to_gpu_leaf::ivy_send_to_gpu_leaf::f`（@0x142aa0628，0x46 字符）。
2. **收集叶实例**：`FUN_1412fa4d0(&Vec, ...)` = `IvyLeafStorage::all_leaves` 的 `Iterator::collect`（fat ptr @0x142aa91b0 → `/rustc/.../core/src/iter/traits/iterator.rs`）。`puStack_68` = 叶数，`plStack_60` = 数据（`free(ptr, count<<3)`）。
3. **SSBO 库查找（AssetSsboLibrary）**：`FUN_140969720(uVar6, &UNK_142c2e248)` —— **IvyLeafSsbo**（NameHash/SsboId = **0x9e3cfa7f2993f406**）；后段 `FUN_140969720(uVar6, &UNK_142c2e280)` —— **IvyFlowerSsbo**（SsboId = **0x1d6de5e69ac50d2a**，与 compute_garden 章互证）。缓冲类型名匹配：`memcmp(名称, &UNK_142a9da84/eb87)`（IvyFlowerSsbo / IvyLeafSsbo 全名串，长度 0x3f/0x3d）。
4. **缓冲重分配**：`plVar16 = 计数 × 0x30`；`plVar15[4] < 需` → 容量不足走 0xb 错误路径；否则 `FUN_140beefa0(&err, *(qword*)*param_4 + 0x10, buf+1, old_size, need, &staging, 1, 0)`（ssbo_library.rs 的缓冲写入/分配原语，file @0x142c2e180）→ 成功则逐叶打包（栈上 uStack_168/160/158 区 = 48B 打包元素）。
5. **失败路径**：`FUN_1428d9760("called Result::unwrap() on an Err value" @0x142aa03b8, 0x2b, ..., &UNK_142aa13f0)` = panic unwrap（文件 ivy_send_to_gpu_leaf.rs）。

**关键事实（O）**：
- **GPU 叶/花元素尺寸 = 0x30 = 48B**（`count * 0x30` 分配）；CPU 侧 IvyLeaf = 0x68B=104B（ivy_leaf_storage.rs，GPU 打包后 48B）。
- **两个 SSBO 键（O，rne-read qword）**：IvyLeafSsbo=0x9e3cfa7f2993f406、IvyFlowerSsbo=0x1d6de5e69ac50d2a。
- SSBO 名称全名（O）：`country_core::systems::ivy::ivy_send_to_gpu_leaf::IvyLeafSsbo` / `...::IvyFlowerSsbo`（0x142a9eb87/0x142a9da84 名表区，即 ssbo 库名表 0x142a9d65d 大表内）。

**跨模块**：消费 `IvyLeafStorage::all_leaves`（0x1408fe280 add 的叶容器）；输出 IvyLeafSsbo/IvyFlowerSsbo（AssetSsboLibrary 名键）；render 域 `instanced_ivy` draw shaders（draw_instanced_ivy_leaf_shader 0x1408dc180 / draw_instanced_ivy_flower_shader 0x1408dc1a0）为 GPU 消费端。
