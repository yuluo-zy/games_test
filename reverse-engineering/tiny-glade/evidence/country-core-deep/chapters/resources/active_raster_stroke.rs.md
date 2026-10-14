# resources/active_raster_stroke.rs

> 文件作用：**活动栅格笔画：ActiveRasterStroke 0x470B 记录（三列 Vec：固化曲线/进行中输入点/平滑后曲线 + 弧长 + 笔刷/模式/blend 标志 + 双脏位图）与 InProgressRasterStroke 流体输入状态机（add_point/solidify）。draw_raster 与 rasterize_terrain_stroke 的输入源。**

函数总计 11：业务 4 / 内联无独立符号 0 / 派生胶水 7

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::active_raster_stroke::ActiveRasterStroke::start_new` | 0x140868d00 | 59 | 重置三列 Vec（cap=0, ptr=dangling）→ 写笔刷@+0x60/模式@+0x64/blend flag@+0x68/计数@+0x6c → add_point(首点) + 推入平滑列 | O | terrain-water\decomp\decomp-0x140868d00.txt
| `country_core::resources::active_raster_stroke::ActiveRasterStroke::resample_and_smooth` | 0x140868e40 | 74 | 固化曲线列 + 剩余流体点（0.5 间距重采样 solidify）合并 → 对全列做 10 遍 0.25/0.5/0.25 三点平滑 | O | terrain-water\decomp\decomp-0x140868e40.txt
| `country_core::resources::active_raster_stroke::InProgressRasterStroke::add_point` | 0x140868fb0 | 107 | 输入点列 push（12B Vec3）；累计弧长@+0x40 加 dist(last,new)；弧长>1.0 触发 solidify_fluid_into 固化前缀并 memmove 剩余点 | O | terrain-water\decomp\decomp-0x140868fb0.txt
| `country_core::resources::active_raster_stroke::InProgressRasterStroke::solidify_fluid_into` | 0x1408691b0 | 138 | 把流体输入点列以 0.5 世界单位定距重采样后并入目标 Vec（<2 点走单点拷贝；尾点合并追加） | O | terrain-water\decomp\decomp-0x1408691b0.txt

## 派生胶水（derive/宏生成；不计独立业务）

`closure`、`default`、`index`、`index_mut`、`last` 等 7 项
