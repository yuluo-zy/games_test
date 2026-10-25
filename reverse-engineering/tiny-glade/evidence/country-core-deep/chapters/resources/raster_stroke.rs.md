# resources/raster_stroke.rs

> 文件作用：**栅格笔刷点集：RasterStroke {points: Arc<[Vec3]>, radius: f32} + iter_points_xz 迭代器。rasterize_stroke 的输入（Map<Iter<Vec3>, iter_points_xz>）。**

函数总计 10：业务 2 / 内联无独立符号 5 / 派生胶水 3

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::raster_stroke::impl$0::fmt<alloc::sync::Arc<slice2$<glam::f32::vec3::Vec3>,alloc::alloc::Global> >` | 0x1409f1c90 | 22 | Debug 格式化（derive 胶水） | O | terrain-water\decomp\decomp-0x1409f1c90.txt（已补；fmt 写 "RasterStroke" 结构名 + 字段 "is_additive"）
| `country_core::resources::raster_stroke::RasterStroke::iter_points_xz` | 0x140a97860 | 59 | points 迭代映射为 (x,z) 对（rasterize_stroke 输入迭代器，50 处内联宿主） | O | hex 手解（NotFound；返回 points 切片 {ptr,end} 范围迭代器：rax=[+8]=ptr、rcx=[+0x10]=len、rdx=ptr+12·len）

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::raster_stroke::RasterStroke::radius`（行 30）— 返回笔刷半径
- `country_core::resources::raster_stroke::RasterStroke::add_point`（行 46）— 追加点（Arc 重建）
- `country_core::resources::raster_stroke::RasterStroke::smooth`（行 63）— 平滑点列（11 处内联）
- `country_core::resources::raster_stroke::RasterStroke::length`（行 70）— 折线总长
- `country_core::resources::raster_stroke::impl$4::from`（行 85）— From 转换（derive）

## 派生胶水（derive/宏生成；不计独立业务）

`clone`、`closure` 等 3 项

## 实现方式（算法级）

- `RasterStroke { points: Arc<[Vec3]>, radius: f32 }`；`iter_points_xz`（0xa97860）= points 迭代映射为 (x,z) 对（12B Vec3 → 8B (x,z)），rasterize_stroke 的输入迭代器（50 处内联宿主）。
- `add_point` = Arc 重建（不可变共享，push 新点）；`smooth` = 点列平滑（11 处内联，宿主同族）；`length` = 折线总长。
- `radius()`/`length()` 为 draw_raster 笔刷/时长消费者。
