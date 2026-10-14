# resources/render/spatial_resolve_offsets.rs

> 文件作用：**SpatialResolveOffsets 资源**——`default_value`（0x140b31d10）把一张 ~0x808 字节的偏移常量表
> （重复 `{f32 偏移, u32 0}` 对，步长 8 字节，来自 const 数据区 _UNK_142aec*）整体拷入结构。
> 语义：空间解析（water/raster 的世界坐标→格子偏移）查找表；仅 default_value 一个业务面。

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::render::spatial_resolve_offsets::impl$0::default_value` | 0x140b31d10 | 24 | 构造：首字段置 1，随后把 0x800+ 字节的 const 偏移表（_UNK_142aeca* 系，步长 8B 的 {值,0} 对）逐项写入 +0x7e*..+0x800 | O | roofs-rt/decomp/decomp-0x140b31d10.txt（54KB，含完整初始化体） |

## 布局/结构要点

- 结构 ≥ 0x808 字节；偏移表为编译期静态常量拷贝（default_value 是 `ConstDefault` 型初始化）。
- 交叉域：空间解析 offset 表与 terrain-water 的 world_raster / 水 mask 网格消费相关。
