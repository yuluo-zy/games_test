# geometry/cube.rs

> 文件作用：**程序化立方体网格 `From<Box> for Mesh`（轴对齐盒）。**（MODULE_GAP 判 0% 覆盖，本节补齐。）

函数总计 3：业务 1 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::geometry::cube::impl$5::from` | 0x140a05530 | 62 | **业务** `From<Box> for Mesh`（3643B）——**确证为 From 转换（非构造器）**：见下方算法节。 | **O** | objdump 0x140a05530-0x140a06370 全文 |

## 内联/无独立符号函数

- `country_core::geometry::cube::impl$2::from`（行 25）— **I**：`From<Cube> for Mesh`（按 reconstruction 6 callers 推断；无独立符号，内联进消费方）。
- `country_core::geometry::cube::Box::new`（行 43）— **I**：`Box::new(min, max)`（内联进 `spawn_human_ref` 0x14089d5a0，见 reconstruction 注）。

## 算法级结论（round 算法深挖，2026-10-15）

### `From<Box> for Mesh`（0x140a05530）全算法

1. **读取 Box**：`rdx = &Box{min: Vec3@+0, max: Vec3@+0xc}`；`min.x`/`min.yz`/`max.z` 分别装载（movss/movsd/movshdup）。
2. **8 角构造**：栈上逐角写 `{pos 12B, uv 8B, color 4B}`（0x18/角）——min/max 三分量组合出 8 角（± 号常量 0x3f800000bf800000=(1.0,-1.0)、0x3f80000000000000=(1.0,0.0)、0x3f8000003f800000=(1.0,1.0) 为 UV 与白色常量；0x142925870=0.5 缩放常量）。
3. **6 面/36 索引**：两嵌套循环（0x140a05a64→0x140a05a82、0x140a05bb9→0x140a05a70）逐面写 6 索引（u32 Vec，grow 0x140562cf0/0x1417967d0 族）。
4. **属性装配**：`Mesh::set_attribute<Vec<[f32;3]>>`（0x140a34c90）×3：`Vertex_Position`（0x142ac6553）、`Vertex_Normal`（0x142ac6562）、`Vertex_Color`（0x142ac6590）；`Mesh::set_attribute<Vec<[f32;2]>>`（0x140a355f0）：`Vertex_Uv`（0x142ac656f）。属性名表含 `BoxVertex_Position` 前缀串（0x142ac6550，mesh format 名，&str 切片用 `Vertex_Position`）。
5. **panic**：4 处分配失败 panic 源均为 `crates/country-core/src/geometry/cube.rs`（0x142ac6508/0x142ac6520/0x142ac6538/0x142ac6578）；收尾 `drop_in_place<Mesh>`（0x140888b30）。

与 reconstruction 注"8 corners / 6 faces / 36 indices；exact vertex normals/UVs I"一致；本轮补：属性含 **Vertex_Color**（原注未列）与 0.5 缩放常量。
