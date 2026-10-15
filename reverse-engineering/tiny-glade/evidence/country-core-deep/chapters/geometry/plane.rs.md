# geometry/plane.rs

> 文件作用：**程序化平面网格 `From<Plane> for Mesh`（单位四边形）。**（MODULE_GAP 判 0% 覆盖，本节补齐。）

函数总计 1：业务 1 / 内联无独立符号 0 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（O 证实/I 推断） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::geometry::plane::impl$1::from` | 0x140a06370 | 20 | **业务** `From<Plane> for Mesh`（1934B）——**确证为 From 转换（非构造器）**：见下方算法节。 | **O** | objdump 0x140a06370-0x140a06b00 全文 |

## 算法级结论（round 算法深挖，2026-10-15）

### `From<Plane> for Mesh`（0x140a06370）全算法

1. **半尺寸**：`xmm1 = size * 0.5`（mulss 0x142925870=0.5）；`xmm0 = -xmm1`（xorps 取负）——Plane {size: Vec2} 每轴半尺寸。
2. **4 角构造**（栈上，0x18/角：{pos 12B, uv 8B, color 4B}）：`(+h, 0, -h)`、`(+h, 0, +h)`、`(-h, 0, +h)`、`(-h, 0, -h)`——**XZ 平面（y=0）四边形**；UV 常量 `(1.0,1.0)`/`(1.0,0.0)`（0x3f8000003f800000、0x3f80000000000000）；color = 0x3f800000（白色 1.0）。
3. **索引**：2 三角形 6 索引（0x140a0647a 起 alloc 0x18 + 索引写入；0x200000003=(2,3) 打包常量）。
4. **属性装配**：与 cube/sphere 同一族：`Vertex_Position`/`Vertex_Normal`/`Vertex_Uv`/`Vertex_Color`（0x142ac6553/0x142ac6562/0x142ac656f/0x142ac6590；set_attribute 0x140a34c90/0x140a355f0）。
5. **panic**：分配失败源 `crates/country-core/src/geometry/plane.rs`（0x142ac6618）；收尾 `drop_in_place<Mesh>`（0x140888b30）。

要点：`Plane` 网格 = XZ 平面单位四边形（y=0），尺寸由 Vec2 控制，2 三角 + 4 属性（含 Vertex_Color，原 reconstruction 未列）。与 cube.rs/sphere.rs 共用同一套属性装配 helper。
