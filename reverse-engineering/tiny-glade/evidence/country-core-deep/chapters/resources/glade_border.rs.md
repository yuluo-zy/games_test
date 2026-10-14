# resources/glade_border.rs

> 文件作用：**小树林边界：GladeBorder 判定世界位置/形状是否在可编辑边界内（is_pos_inside=跳板→is_shape_inside 主体；依赖 utils Circle2d/CurveU——u∉[0,1) panic 'Tried to init CurveU from an invalid u: '）。**

函数总计 5：业务 3 / 内联无独立符号 2 / 派生胶水 0

## 业务函数

| 函数 | 地址 | 源码行 | 作用（猜测→待证实） | 逆向结论 | 证据 |
|---|---|---|---|---|---|
| `country_core::resources::glade_border::GladeBorder::is_pos_inside` | 0x140ad2950 | 40 | 跳板：test [rax],1 后跳入 is_shape_inside 共享主体（世界位置界内判定） | O | terrain-water/DIGEST.md
| `country_core::resources::glade_border::GladeBorder::is_shape_inside` | 0x140ad2970 | 54 | 形状（Circle2d 等）界内判定主体：CurveU 边界校验 + 形状采样比较 | O | terrain-water\decomp\decomp-0x140ad2970.txt
| `country_core::resources::glade_border::GladeBorder::search_lerped_shape_that_fits_into_glade` | 0x140ad2ce0 | 95 | 在界内搜索可容纳的 lerp 缩放后形状（形状放置缩放搜索，6.6KB） | O | terrain-water\decomp\decomp-0x140ad2ce0.txt

## 内联/无独立符号函数（名称仍揭示意图；逻辑并入调用点）

- `country_core::resources::glade_border::GladeBorder::is_curve2_inside`（行 44）— 曲线（Curve2）界内判定
- `country_core::resources::glade_border::init_glade_border`（行 128）— 初始化边界资源（曲线/形状数据）
