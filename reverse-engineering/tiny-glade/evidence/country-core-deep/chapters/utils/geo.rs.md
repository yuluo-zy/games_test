# utils/geo.rs

> 文件作用：**曲线/网格几何重建**。`update_curve_mesh`（4 泛型实例）把曲线点迭代器生成曲线网格
> （bevy `Vertex_Position`/`Vertex_Color` + ribbon 生成 FUN_1412f7260 + u32 索引）；`rewire_enclosed_curve`
> 切片重连闭合曲线（端点 ε 去重，复用私有 `utils::curve` crate）；`mesh_from_pts` 点集→三角网格
> （u32 顺序索引，SIMD 展开）。
> 证据：各实例反编译 + panic 源 `crates/country-core/src/utils/geo.rs`、`MEOW####` 自定义 panic 前缀。

函数总计 9：业务 6 / 内联无独立符号 0 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `update_curve_mesh<IntoIter<Vec3>>` | 0x14157eac0 | 10 | 点迭代器→曲线网格 | **O** | Vertex_Position/Vertex_Color + FUN_1412f7260 ribbon |
| `update_curve_mesh<Copied<Iter<Vec3>>>` | 0x14157f130 | 10 | 点切片→曲线网格 | **O** | 同（直接拷贝点） |
| `update_curve_mesh<circle signifier 闭包>` | 0x14157eda0 | 10 | 圆形编辑线→曲线网格 | **O** | wall::shapes::circle 闭包实例化 |
| `update_curve_mesh<rectangle signifier 闭包>` | 0x14157ef70 | 10 | 矩形编辑线→曲线网格 | **O** | wall::shapes::rectangle 闭包实例化 |
| `rewire_enclosed_curve` | 0x140a6ba50 | 29 | 闭合曲线重连 | **O** | 阈值哨兵→空；FUN_141466bb0 切片 + ε 去重(_UNK_142925984) + FUN_141469430；私有 utils::curve panic |
| `mesh_from_pts` | 0x140a6c120 | 95 | 点集→三角网格 | **O** | pts_len>0 断言；Vertex_Position/Vertex_Color；u32 顺序索引 SIMD |

## 派生胶水（3 项：closure 等）
