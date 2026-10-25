# utils/geo.rs

> 文件作用：**曲线/网格几何重建（游戏自有）**。`update_curve_mesh`（4 泛型实例）把曲线点
> 迭代器生成**曲线 ribbon 网格**（bevy `Vertex_Position`/`Vertex_Color` + ribbon 生成
> FUN_1412f7260 + u32 索引）；`rewire_enclosed_curve` 切片重连闭合曲线（端点 ε 去重，复用
> 私有 `utils::curve` crate 的 slice/resample）；`mesh_from_pts` 点集→三角网格（u32 顺序索引，
> SIMD 展开）。共享 `utils` 的 `geometry`/`curve` 是数学类型面（Curve/Rectangle2d/Aabb2 等），
> **不含网格构建函数**——本文件全为 country-core 特有。

## 核心算法

```
update_curve_mesh(points_iter) -> Mesh:
  逐点推 Vertex_Position（Vec3）+ Vertex_Color；FUN_1412f7260 生成 ribbon 侧带；
  u32 顺序索引；null 输出 panic `MEOW####`（geo.rs 自定义前缀）
rewire_enclosed_curve(curve, ε):
  阈值==0/哨兵 → 返回空；FUN_141466bb0 曲线切片（私有 utils::curve crate，
  panic 源 crates/utils/src/curve/mod.rs）；端点 ε 去重（常量 _UNK_142925984）；
  FUN_141469430 重建闭合曲线
mesh_from_pts(pts):
  assert pts_len>0；拷贝 Vec3；Vertex_Position/Vertex_Color；u32 顺序索引
  SIMD 常量 0,1,2,3 / 4,4,4,4 / 8,8,8,8（8 元素展开步长）；输出
  {count, indices_ptr, count, 哨兵 0x8000000000000000, attributes…}
```

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
