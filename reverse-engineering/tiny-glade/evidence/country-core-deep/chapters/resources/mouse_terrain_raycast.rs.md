# resources/mouse_terrain_raycast.rs

> 文件作用：**鼠标→地形拾取链：CameraCursorRay {tag u32@0, pos@4, dir@0x10, length@0x1c} 相机光标射线 + camera_mouse_ray 屏幕→世界射线 + mouse_terrain_raycast 地形拾取（xz 平面快路径 / TerrainHeightsData::intersect_ray 高度场求交 / y=0 平面比距取近 / closure$0 实例网格精化）。**

函数总计 4：业务 3 / 内联无独立符号 0 / 派生胶水 1

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::mouse_terrain_raycast::CameraCursorRay::intersect_with_xz_plane` | 0x1408ef180 | 27 | t=(y-pos.y)/dir.y；返回 pos+t*dir（xz 平面交点，pos/dir 从 +4/+0x10 读） | O | hex 手解（NotFound；`t=(plane_y−pos.y)/dir.y`，SIMD `pos+t*dir` 全 3D 交点；pos@0/dir@0xc=资源相对 +4/+0x10，见 round3/DANGLING-FILL-REPORT.md）
| `country_core::resources::mouse_terrain_raycast::camera_mouse_ray` | 0x1408ef1c0 | 35 | 由 CursorPositionSS 屏幕位置经 Ray::from_mouse_ss 构建世界射线（utils::geometry::ray 依赖） | O | terrain-water\decomp\decomp-0x1408ef1c0.txt
| `country_core::resources::mouse_terrain_raycast::mouse_terrain_raycast` | 0x1408ef320 | 63 | CameraCursorRay 有效→xz 平面解；否则 TerrainHeightsData::intersect_ray 高度场求交（32 步行进+3 二分），与 y=0 平面交点比距取近，写入 pos/dir-delta/length；closure$0 实例网格精化 | O | terrain-water\decomp\decomp-0x1408ef320.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure` 等 1 项

## 实现方式（算法级）

- `intersect_with_xz_plane`（0x8ef180）：t=(y−pos.y)/dir.y；返回 pos+t·dir。
- `camera_mouse_ray`（0x8ef1c0）：CursorPositionSS 屏幕位置 → `Ray::from_mouse_ss` 世界射线。
- `mouse_terrain_raycast`（0x8ef320）：CameraCursorRay 有效 → xz 平面解；否则 `TerrainHeightsData::intersect_ray`（32 步行进+3 二分）；与 y=0 平面交点比距取近；closure$0 网格精化。写 pos/dir-delta/length。
